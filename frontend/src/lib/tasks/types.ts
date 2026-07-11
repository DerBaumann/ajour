import z from 'zod';
import type { $ZodIssue } from 'zod/v4/core';

export const Priority = z.enum(['very_high', 'high', 'medium', 'low']);
export type Priority = z.infer<typeof Priority>;

export const CreateTask = z.object({
	name: z.string().max(50),
	description: z.string().max(300).optional(),
	priority: Priority,
	start: z.string(),
	deadline: z.string().optional()
});
export type CreateTask = z.infer<typeof CreateTask>;

export const Task = z.object({
	id: z.number(),
	name: z.string(),
	description: z.string().optional(),
	completed: z.boolean(),
	priority: Priority,
	start: z.coerce.date(),
	deadline: z.coerce.date().optional(),
	archived_at: z.coerce.date().optional(),
	created_at: z.coerce.date()
});
export type Task = z.infer<typeof Task>;

export type TaskError =
	| { type: 'zod_error'; issues: $ZodIssue[] }
	| { type: 'http_error'; status: number; message?: string }
	| { type: 'parse_error'; message: string };
