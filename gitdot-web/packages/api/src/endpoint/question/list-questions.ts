import { z } from "zod";
import { page, QuestionResource } from "../../resource";
import type { Endpoint } from "../endpoint";

export const ListQuestionsRequest = z.object({
  cursor: z.string().optional(),
  limit: z.number().int().positive().optional(),
});
export type ListQuestionsRequest = z.infer<typeof ListQuestionsRequest>;

export const ListQuestionsResponse = page(QuestionResource);
export type ListQuestionsResponse = z.infer<typeof ListQuestionsResponse>;

export const ListQuestions = {
  path: "/repository/{owner}/{repo}/questions",
  method: "GET",
  request: ListQuestionsRequest,
  response: ListQuestionsResponse,
} as const satisfies Endpoint;
export type ListQuestions = typeof ListQuestions;
