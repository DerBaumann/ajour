export type Priority = 'very_high' | 'high' | 'medium' | 'low';

export type Task = {
	id: number;
	name: string;
	description: string | undefined;
	completed: boolean;
	priority: Priority;
	start: Date;
	deadline: Date | undefined;
	archived_at: Date | undefined;
	created_at: Date;
};

export type CreateTaskRequest = {
	name: string;
	description: string | undefined;
	priority: Priority;
	start: Date;
	deadline: Date | undefined;
};

// pub struct CreateTaskRequest {
//     #[validate(length(max = 50))]
//     pub name: String,
//     #[validate(length(max = 300))]
//     #[serde(deserialize_with = "blank_as_none")]
//     pub description: Option<String>,
//     pub priority: Priority,
//     #[validate(custom(function = "validate_datetime"))]
//     pub start: String,
//     #[validate(custom(function = "validate_datetime"))]
//     #[serde(deserialize_with = "blank_as_none")]
//     pub deadline: Option<String>,
// }
