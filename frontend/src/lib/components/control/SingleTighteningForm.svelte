<script lang="ts">
	import { api } from '$lib/api/client';
	import { deviceState } from '$lib/stores/device';
	import { showToast } from '$lib/stores/ui';
	import { Section, Button, FormField } from '$lib/components/ui';
	import { getPsetTargets, formatErrorMessage, validateRange } from '$lib/utils';
	import type { Pset, TighteningRequest, ToolDirection } from '$lib/types';

	interface Props {
		currentPset: Pset | undefined;
	}

	let { currentPset }: Props = $props();

	let usePsetValues = $state(true);
	let resultMode: 'auto' | 'ok' | 'nok' = $state('auto');
	let tighteningPayload = $state({
		torque: 12.5,
		angle: 40.0
	});
	let isSubmitting = $state(false);
	let isUpdatingDirection = $state(false);
	let selectedDirection = $state<ToolDirection>('CW');
	let validationErrors = $state({
		torque: '',
		angle: ''
	});

	const currentPsetTargets = $derived(
		currentPset ? getPsetTargets(currentPset) : null
	);
	const isToolEnabled = $derived($deviceState?.tool_enabled ?? true);

	const isFormValid = $derived(
		usePsetValues || (!validationErrors.torque && !validationErrors.angle)
	);

	// Real-time validation using $effect
	$effect(() => {
		selectedDirection = $deviceState?.tool_direction ?? 'CW';
	});

	$effect(() => {
		if (!usePsetValues) {
			validationErrors.torque = validateRange(
				tighteningPayload.torque,
				0,
				100,
				'Torque'
			);
			validationErrors.angle = validateRange(
				tighteningPayload.angle,
				0,
				360,
				'Angle'
			);
		} else {
			// Clear errors when using PSET values
			validationErrors.torque = '';
			validationErrors.angle = '';
		}
	});

	async function handleDirectionChange(direction: ToolDirection) {
		if (direction === ($deviceState?.tool_direction ?? 'CW')) {
			selectedDirection = direction;
			return;
		}

		selectedDirection = direction;
		isUpdatingDirection = true;
		try {
			await api.setToolDirection(direction);
		} catch (error) {
			selectedDirection = $deviceState?.tool_direction ?? 'CW';
			showToast({ type: 'error', message: formatErrorMessage('set tool direction', error) });
		} finally {
			isUpdatingDirection = false;
		}
	}

	async function handleSubmit() {
		if (!isToolEnabled) {
			showToast({
				type: 'warning',
				message: 'Tool is disabled. Enable the tool before simulating tightening.'
			});
			return;
		}

		isSubmitting = true;
		try {
			let payload: TighteningRequest = {};

			if (!usePsetValues) {
				payload.torque = tighteningPayload.torque;
				payload.angle = tighteningPayload.angle;

				if (resultMode !== 'auto') {
					payload.ok = resultMode === 'ok';
				}
			}

			await api.simulateTightening(payload);
			showToast({ type: 'success', message: 'Tightening simulated!' });
		} catch (error) {
			showToast({ type: 'error', message: formatErrorMessage('simulate tightening', error) });
		} finally {
			isSubmitting = false;
		}
	}
</script>

<Section
	title="Single Tightening"
	description="Run an ad-hoc cycle using the configured PSET or override values manually."
>
	<form
		onsubmit={(e) => {
			e.preventDefault();
			handleSubmit();
		}}
		class="space-y-6"
	>
		{#if !isToolEnabled}
			<div
				class="flex items-start gap-2 rounded-md bg-surface-100-800-token p-3 text-sm text-warning-600"
			>
				<span aria-hidden="true">⚠️</span>
				<span>Tool is disabled. Single tightening simulation is unavailable.</span>
			</div>
		{/if}

		<div class="rounded-lg border border-surface-200-700-token bg-surface-100-800-token p-4">
			<div class="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
				<div class="space-y-3">
					<div>
						<p class="text-xs uppercase tracking-wide text-surface-600-300-token">
							Tool Direction
						</p>
						<p class="mt-1 text-sm text-surface-600 dark:text-surface-400">
							CCW means relay 22 is active. CW means relay 22 is off.
						</p>
					</div>
					<div class="flex items-center gap-2 text-sm font-medium text-surface-700 dark:text-surface-300">
						<span
							class="inline-flex h-3 w-3 rounded-full"
							class:bg-success-500={isToolEnabled}
							class:bg-error-500={!isToolEnabled}
						></span>
						<span>Enabled</span>
					</div>
					<div class="inline-flex rounded-lg border border-surface-200-700-token bg-surface-50-900-token p-1">
						<label class="cursor-pointer">
							<input
								class="sr-only"
								type="radio"
								name="tool-direction"
								value="CW"
								checked={selectedDirection === 'CW'}
								disabled={isUpdatingDirection}
								onchange={() => handleDirectionChange('CW')}
							/>
							<span
								class="block rounded px-4 py-2 text-sm font-semibold transition-colors"
								class:bg-primary-500={selectedDirection === 'CW'}
								class:text-white={selectedDirection === 'CW'}
								class:opacity-60={selectedDirection !== 'CW'}
							>
								CW
							</span>
						</label>
						<label class="cursor-pointer">
							<input
								class="sr-only"
								type="radio"
								name="tool-direction"
								value="CCW"
								checked={selectedDirection === 'CCW'}
								disabled={isUpdatingDirection}
								onchange={() => handleDirectionChange('CCW')}
							/>
							<span
								class="block rounded px-4 py-2 text-sm font-semibold transition-colors"
								class:bg-primary-500={selectedDirection === 'CCW'}
								class:text-white={selectedDirection === 'CCW'}
								class:opacity-60={selectedDirection !== 'CCW'}
							>
								CCW
							</span>
						</label>
					</div>
				</div>

				<Button
					type="submit"
					disabled={isSubmitting || !isFormValid || !isToolEnabled || isUpdatingDirection}
					class="w-full lg:w-auto"
				>
					{isSubmitting ? 'Simulating...' : 'Tighten'}
				</Button>
			</div>
		</div>

		<!-- Toggle between PSET and Manual -->
		<div
			class="inline-flex rounded-lg border border-surface-200-700-token bg-surface-100-800-token p-1"
		>
			<button
				type="button"
				class="px-4 py-2 text-sm font-semibold transition-colors rounded"
				class:bg-primary-500={usePsetValues}
				class:text-white={usePsetValues}
				class:opacity-60={!usePsetValues}
				onclick={() => (usePsetValues = true)}
			>
				Use PSET Values
			</button>
			<button
				type="button"
				class="px-4 py-2 text-sm font-semibold transition-colors rounded"
				class:bg-primary-500={!usePsetValues}
				class:text-white={!usePsetValues}
				class:opacity-60={usePsetValues}
				onclick={() => (usePsetValues = false)}
			>
				Manual Override
			</button>
		</div>

		{#if usePsetValues}
			<!-- PSET Mode -->
			<div
				class="rounded-lg border border-surface-200-700-token bg-surface-100-800-token p-4"
			>
				{#if currentPsetTargets}
					<p class="text-xs uppercase tracking-wide text-surface-600-300-token">
						PSET Target
					</p>
					<div class="mt-2 grid grid-cols-1 gap-3 sm:grid-cols-2">
						<div>
							<p class="text-sm opacity-70">Torque</p>
							<p class="text-xl font-semibold">
								{currentPsetTargets.torque} Nm
							</p>
						</div>
						<div>
							<p class="text-sm opacity-70">Angle</p>
							<p class="text-xl font-semibold">
								{currentPsetTargets.angle}°
							</p>
						</div>
					</div>
					<p class="mt-3 text-sm opacity-70">
						Outcome determined automatically by the FSM.
					</p>
				{:else}
					<p class="text-sm opacity-70">Select a PSET to preview its targets.</p>
				{/if}
			</div>
		{:else}
			<!-- Manual Override Mode -->
			<div class="grid gap-4 md:grid-cols-2">
				<FormField
					label="Torque (Nm)"
					type="number"
					bind:value={tighteningPayload.torque}
					step="0.1"
					min={0}
					max={100}
					error={validationErrors.torque}
				/>
				<FormField
					label="Angle (degrees)"
					type="number"
					bind:value={tighteningPayload.angle}
					step="0.1"
					min={0}
					max={360}
					error={validationErrors.angle}
				/>
			</div>

			<FormField
				label="Result Mode"
				type="select"
				bind:value={resultMode}
				options={[
					{ value: 'auto', label: 'Auto (FSM determines)' },
					{ value: 'ok', label: 'Force OK' },
					{ value: 'nok', label: 'Force NOK' }
				]}
			/>
		{/if}

	</form>
</Section>
