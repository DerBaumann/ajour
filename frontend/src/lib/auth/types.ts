import z from 'zod';

export const RegistrationCreds = z
	.object({
		name: z.string(),
		email: z.email(),
		password: z.string(),
		passwordRepeat: z.string()
	})
	.refine(({ password, passwordRepeat }) => password === passwordRepeat, {
		error: 'Passwords dont match'
	});
export type RegistrationCreds = z.infer<typeof RegistrationCreds>;

export const LoginCreds = z.object({
	email: z.email(),
	password: z.string()
});
export type LoginCreds = z.infer<typeof LoginCreds>;
