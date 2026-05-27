"use client";

import type { QuestionResource } from "gitdot-api";
import { useMemo, useState } from "react";
import { QuestionRow } from "./ui/question-row";
import { QuestionsHeader } from "./ui/questions-header";
import { processQuestions } from "./util";

export type QuestionsFilter = "popular" | "unanswered" | "all";
export type QuestionsSort =
  | "created-asc"
  | "created-desc"
  | "updated-asc"
  | "updated-desc"
  | "vote-asc"
  | "vote-desc";

export function PageClient({
  owner,
  repo,
  questions,
}: {
  owner: string;
  repo: string;
  questions: QuestionResource[] | null;
}) {
  const [filter, setFilter] = useState<QuestionsFilter>("popular");
  const [sort, setSort] = useState<QuestionsSort>("created-asc");

  const processedQuestions = useMemo(
    () => processQuestions(questions ?? [], filter, sort),
    [questions, filter, sort],
  );

  if (!questions) return null;

  return (
    <div className="flex flex-col">
      <QuestionsHeader
        owner={owner}
        repo={repo}
        filter={filter}
        setFilter={setFilter}
        sort={sort}
        setSort={setSort}
      />
      {processedQuestions.map((question) => (
        <QuestionRow
          key={question.id}
          owner={owner}
          repo={repo}
          question={question}
        />
      ))}
    </div>
  );
}
