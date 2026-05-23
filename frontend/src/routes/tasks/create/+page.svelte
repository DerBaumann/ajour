<!-- TODO: Cleanup -->
<script lang="ts">
	import { resolve } from '$app/paths';
	import type { DateValue } from '@skeletonlabs/skeleton-svelte';
	import { DatePicker, parseDate, Portal } from '@skeletonlabs/skeleton-svelte';

	let start: DateValue[] = $state([parseDate(new Date())]);
	let deadline: DateValue[] = $state([]);
	// let locale = $state('');
	//
	// onMount(() => {
	// 	locale = navigator.language;
	// });

	function toApiDate(date: DateValue): string {
		const year = date.year;
		const month = String(date.month).padStart(2, '0');
		const day = String(date.day).padStart(2, '0');
		return `${year}-${month}-${day}`;
	}
</script>

<h1 class="h1">Neue Aufgabe</h1>

<p>
	<a href={resolve('/tasks')} class="btn preset-outlined-secondary-500">Zurück </a>
</p>

<form class="w-full max-w-md space-y-4 p-4" method="POST">
	<fieldset class="space-y-4">
		<!-- Input -->
		<label class="label">
			<span class="label-text">Name</span>
			<input class="input" type="text" name="name" required />
		</label>
		<!-- Textarea -->
		<label class="label">
			<span class="label-text">Beschreibung</span>
			<textarea
				class="textarea rounded-container"
				rows="4"
				placeholder="Optionale Beschreibung"
				name="description"
			></textarea>
		</label>
	</fieldset>

	<fieldset class="space-y-2">
		<p>Priorität</p>
		<label class="flex items-center space-x-2">
			<input class="radio" type="radio" checked name="priority" value="very_high" />
			<p>Sehr hoch</p>
		</label>
		<label class="flex items-center space-x-2">
			<input class="radio" type="radio" name="priority" value="high" />
			<p>Hoch</p>
		</label>
		<label class="flex items-center space-x-2">
			<input class="radio" type="radio" name="priority" value="medium" />
			<p>Mittel</p>
		</label>
		<label class="flex items-center space-x-2">
			<input class="radio" type="radio" name="priority" value="low" />
			<p>Niedrig</p>
		</label>
	</fieldset>

	<!-- TODO: Dynamic locale -->
	<DatePicker required value={start} onValueChange={(e) => (start = e.value)}>
		<DatePicker.Label>Start</DatePicker.Label>
		<DatePicker.Control>
			<DatePicker.Input placeholder="yyyy-mm-dd" />
			<DatePicker.Trigger />
		</DatePicker.Control>
		<Portal>
			<DatePicker.Positioner>
				<DatePicker.Content>
					<DatePicker.View view="day">
						<DatePicker.Context>
							{#snippet children(datePicker)}
								<DatePicker.ViewControl>
									<DatePicker.PrevTrigger />
									<DatePicker.ViewTrigger>
										<DatePicker.RangeText />
									</DatePicker.ViewTrigger>
									<DatePicker.NextTrigger />
								</DatePicker.ViewControl>
								<DatePicker.Table>
									<DatePicker.TableHead>
										<DatePicker.TableRow>
											{#each datePicker().weekDays as weekDay, id (id)}
												<DatePicker.TableHeader>{weekDay.short}</DatePicker.TableHeader>
											{/each}
										</DatePicker.TableRow>
									</DatePicker.TableHead>
									<DatePicker.TableBody>
										{#each datePicker().weeks as week, id (id)}
											<DatePicker.TableRow>
												{#each week as day, id (id)}
													<DatePicker.TableCell value={day}>
														<DatePicker.TableCellTrigger>{day.day}</DatePicker.TableCellTrigger>
													</DatePicker.TableCell>
												{/each}
											</DatePicker.TableRow>
										{/each}
									</DatePicker.TableBody>
								</DatePicker.Table>
							{/snippet}
						</DatePicker.Context>
					</DatePicker.View>
					<DatePicker.View view="month">
						<DatePicker.Context>
							{#snippet children(datePicker)}
								<DatePicker.ViewControl>
									<DatePicker.PrevTrigger />
									<DatePicker.ViewTrigger>
										<DatePicker.RangeText />
									</DatePicker.ViewTrigger>
									<DatePicker.NextTrigger />
								</DatePicker.ViewControl>
								<DatePicker.Table>
									<DatePicker.TableBody>
										{#each datePicker().getMonthsGrid( { columns: 4, format: 'short' } ) as months, id (id)}
											<DatePicker.TableRow>
												{#each months as month, id (id)}
													<DatePicker.TableCell value={month.value}>
														<DatePicker.TableCellTrigger>{month.label}</DatePicker.TableCellTrigger>
													</DatePicker.TableCell>
												{/each}
											</DatePicker.TableRow>
										{/each}
									</DatePicker.TableBody>
								</DatePicker.Table>
							{/snippet}
						</DatePicker.Context>
					</DatePicker.View>
					<DatePicker.View view="year">
						<DatePicker.Context>
							{#snippet children(datePicker)}
								<DatePicker.ViewControl>
									<DatePicker.PrevTrigger />
									<DatePicker.ViewTrigger>
										<DatePicker.RangeText />
									</DatePicker.ViewTrigger>
									<DatePicker.NextTrigger />
								</DatePicker.ViewControl>
								<DatePicker.Table>
									<DatePicker.TableBody>
										{#each datePicker().getYearsGrid({ columns: 4 }) as years, id (id)}
											<DatePicker.TableRow>
												{#each years as year, id (id)}
													<DatePicker.TableCell value={year.value}>
														<DatePicker.TableCellTrigger>{year.label}</DatePicker.TableCellTrigger>
													</DatePicker.TableCell>
												{/each}
											</DatePicker.TableRow>
										{/each}
									</DatePicker.TableBody>
								</DatePicker.Table>
							{/snippet}
						</DatePicker.Context>
					</DatePicker.View>
				</DatePicker.Content>
			</DatePicker.Positioner>
		</Portal>
	</DatePicker>
	{#if start?.[0]}
		<input type="hidden" name="start" value={toApiDate(start[0])} />
	{/if}

	<!-- TODO: Dynamic locale -->
	<DatePicker value={deadline} onValueChange={(e) => (deadline = e.value)}>
		<DatePicker.Label>Deadline</DatePicker.Label>
		<DatePicker.Control>
			<DatePicker.Input placeholder="yyyy-mm-dd" />
			<DatePicker.Trigger />
		</DatePicker.Control>
		<Portal>
			<DatePicker.Positioner>
				<DatePicker.Content>
					<DatePicker.View view="day">
						<DatePicker.Context>
							{#snippet children(datePicker)}
								<DatePicker.ViewControl>
									<DatePicker.PrevTrigger />
									<DatePicker.ViewTrigger>
										<DatePicker.RangeText />
									</DatePicker.ViewTrigger>
									<DatePicker.NextTrigger />
								</DatePicker.ViewControl>
								<DatePicker.Table>
									<DatePicker.TableHead>
										<DatePicker.TableRow>
											{#each datePicker().weekDays as weekDay, id (id)}
												<DatePicker.TableHeader>{weekDay.short}</DatePicker.TableHeader>
											{/each}
										</DatePicker.TableRow>
									</DatePicker.TableHead>
									<DatePicker.TableBody>
										{#each datePicker().weeks as week, id (id)}
											<DatePicker.TableRow>
												{#each week as day, id (id)}
													<DatePicker.TableCell value={day}>
														<DatePicker.TableCellTrigger>{day.day}</DatePicker.TableCellTrigger>
													</DatePicker.TableCell>
												{/each}
											</DatePicker.TableRow>
										{/each}
									</DatePicker.TableBody>
								</DatePicker.Table>
							{/snippet}
						</DatePicker.Context>
					</DatePicker.View>
					<DatePicker.View view="month">
						<DatePicker.Context>
							{#snippet children(datePicker)}
								<DatePicker.ViewControl>
									<DatePicker.PrevTrigger />
									<DatePicker.ViewTrigger>
										<DatePicker.RangeText />
									</DatePicker.ViewTrigger>
									<DatePicker.NextTrigger />
								</DatePicker.ViewControl>
								<DatePicker.Table>
									<DatePicker.TableBody>
										{#each datePicker().getMonthsGrid( { columns: 4, format: 'short' } ) as months, id (id)}
											<DatePicker.TableRow>
												{#each months as month, id (id)}
													<DatePicker.TableCell value={month.value}>
														<DatePicker.TableCellTrigger>{month.label}</DatePicker.TableCellTrigger>
													</DatePicker.TableCell>
												{/each}
											</DatePicker.TableRow>
										{/each}
									</DatePicker.TableBody>
								</DatePicker.Table>
							{/snippet}
						</DatePicker.Context>
					</DatePicker.View>
					<DatePicker.View view="year">
						<DatePicker.Context>
							{#snippet children(datePicker)}
								<DatePicker.ViewControl>
									<DatePicker.PrevTrigger />
									<DatePicker.ViewTrigger>
										<DatePicker.RangeText />
									</DatePicker.ViewTrigger>
									<DatePicker.NextTrigger />
								</DatePicker.ViewControl>
								<DatePicker.Table>
									<DatePicker.TableBody>
										{#each datePicker().getYearsGrid({ columns: 4 }) as years, id (id)}
											<DatePicker.TableRow>
												{#each years as year, id (id)}
													<DatePicker.TableCell value={year.value}>
														<DatePicker.TableCellTrigger>{year.label}</DatePicker.TableCellTrigger>
													</DatePicker.TableCell>
												{/each}
											</DatePicker.TableRow>
										{/each}
									</DatePicker.TableBody>
								</DatePicker.Table>
							{/snippet}
						</DatePicker.Context>
					</DatePicker.View>
				</DatePicker.Content>
			</DatePicker.Positioner>
		</Portal>
	</DatePicker>
	{#if deadline?.[0]}
		<input type="hidden" name="deadline" value={toApiDate(deadline[0])} />
	{/if}

	<fieldset class="flex justify-end">
		<button type="submit" class="btn preset-filled-primary-500">Speichern</button>
	</fieldset>
</form>
