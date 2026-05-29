use async_trait::async_trait;

use crate::{
    dto::{
        AnswerResponse, CommentResponse, CreateAnswerCommentRequest, CreateAnswerRequest,
        CreateQuestionCommentRequest, CreateQuestionRequest, GetQuestionRequest,
        ListQuestionsRequest, Page, QuestionResponse, UpdateAnswerRequest, UpdateCommentRequest,
        UpdateQuestionRequest, VoteAnswerRequest, VoteCommentRequest, VoteQuestionRequest,
        VoteResponse,
    },
    error::{OptionNotFoundExt, QuestionError},
    model::VoteTarget,
    repository::{
        PgQuestionRepository, PgRepositoryRepository, QuestionRepository, RepositoryRepository,
    },
    util::cursor,
};

/// Manages a repository's Q&A: questions, their answers, comments on both, and
/// voting across all three.
///
/// Questions are scoped to a repository and identified by a repo-scoped number;
/// answers and comments hang off questions, and votes can target a question, an
/// answer, or a comment.
#[async_trait]
pub trait QuestionService: Send + Sync + 'static {
    /// Creates a new question in a repository with the given title and body.
    ///
    /// # Errors
    /// - [`QuestionError::NotFound`] if the repository
    ///   does not exist.
    async fn create_question(
        &self,
        request: CreateQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError>;

    /// Updates an existing question's title and body.
    ///
    /// # Errors
    /// - [`QuestionError::NotFound`] if the repository or
    ///   the question does not exist.
    async fn update_question(
        &self,
        request: UpdateQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError>;

    /// Returns a single question by its repo-scoped number.
    ///
    /// The requesting user's id is passed through so the response can reflect
    /// that user's vote state.
    ///
    /// # Errors
    /// - [`QuestionError::NotFound`] if the repository or
    ///   the question does not exist.
    async fn get_question(
        &self,
        request: GetQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError>;

    /// Lists a repository's questions as a cursor-paginated page.
    ///
    /// The requesting user's id is passed through for per-user vote state, and a
    /// `next_cursor` is returned when more pages remain.
    ///
    /// # Errors
    /// - [`QuestionError::NotFound`] if the repository
    ///   does not exist.
    async fn list_questions(
        &self,
        request: ListQuestionsRequest,
    ) -> Result<Page<QuestionResponse>, QuestionError>;

    /// Creates an answer to a question.
    ///
    /// # Errors
    /// - [`QuestionError::NotFound`] if the target
    ///   question does not exist.
    async fn create_answer(
        &self,
        request: CreateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError>;

    /// Updates the body of an existing answer.
    ///
    /// # Errors
    /// - [`QuestionError::NotFound`] if the answer does
    ///   not exist.
    async fn update_answer(
        &self,
        request: UpdateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError>;

    /// Creates a comment on a question.
    ///
    /// # Errors
    /// - [`QuestionError::NotFound`] if the target
    ///   question does not exist.
    async fn create_question_comment(
        &self,
        request: CreateQuestionCommentRequest,
    ) -> Result<CommentResponse, QuestionError>;

    /// Creates a comment on an answer.
    async fn create_answer_comment(
        &self,
        request: CreateAnswerCommentRequest,
    ) -> Result<CommentResponse, QuestionError>;

    /// Updates the body of an existing comment (on a question or answer).
    ///
    /// # Errors
    /// - [`QuestionError::NotFound`] if the comment does
    ///   not exist.
    async fn update_comment(
        &self,
        request: UpdateCommentRequest,
    ) -> Result<CommentResponse, QuestionError>;

    /// Casts (or changes/clears) the requesting user's vote on a question.
    ///
    /// # Errors
    /// - [`QuestionError::NotFound`] if the question does
    ///   not exist.
    async fn vote_question(
        &self,
        request: VoteQuestionRequest,
    ) -> Result<VoteResponse, QuestionError>;

    /// Casts (or changes/clears) the requesting user's vote on an answer.
    async fn vote_answer(&self, request: VoteAnswerRequest) -> Result<VoteResponse, QuestionError>;

    /// Casts (or changes/clears) the requesting user's vote on a comment.
    async fn vote_comment(
        &self,
        request: VoteCommentRequest,
    ) -> Result<VoteResponse, QuestionError>;
}

#[derive(Debug, Clone)]
pub struct QuestionServiceImpl<Q, R>
where
    Q: QuestionRepository,
    R: RepositoryRepository,
{
    question_repo: Q,
    repo_repo: R,
}

impl QuestionServiceImpl<PgQuestionRepository, PgRepositoryRepository> {
    pub fn new(question_repo: PgQuestionRepository, repo_repo: PgRepositoryRepository) -> Self {
        Self {
            question_repo,
            repo_repo,
        }
    }
}

#[crate::instrument_all(level = "debug")]
#[async_trait]
impl<Q, R> QuestionService for QuestionServiceImpl<Q, R>
where
    Q: QuestionRepository,
    R: RepositoryRepository,
{
    async fn create_question(
        &self,
        request: CreateQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref(), None)
            .await?
            .or_not_found("repository", request.get_repo_path())?;

        let question = self
            .question_repo
            .create_question(
                request.author_id,
                repository.id,
                &request.title,
                &request.body,
            )
            .await?;

        Ok(question.into())
    }

    async fn update_question(
        &self,
        request: UpdateQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref(), None)
            .await?
            .or_not_found("repository", request.get_repo_path())?;

        let question = self
            .question_repo
            .update_question(repository.id, request.number, &request.title, &request.body)
            .await?
            .or_not_found("question", request.get_question_path())?;

        Ok(question.into())
    }

    async fn get_question(
        &self,
        request: GetQuestionRequest,
    ) -> Result<QuestionResponse, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref(), None)
            .await?
            .or_not_found("repository", request.get_repo_path())?;

        let question = self
            .question_repo
            .get_question(repository.id, request.number, request.user_id)
            .await?
            .or_not_found("question", request.get_question_path())?;

        Ok(question.into())
    }

    async fn list_questions(
        &self,
        request: ListQuestionsRequest,
    ) -> Result<Page<QuestionResponse>, QuestionError> {
        let repository = self
            .repo_repo
            .get(request.owner.as_ref(), request.repo.as_ref(), None)
            .await?
            .or_not_found("repository", request.get_repo_path())?;

        let (questions, next_cursor) = self
            .question_repo
            .list_questions(
                repository.id,
                request.user_id,
                request.cursor,
                request.limit as i64,
            )
            .await?;

        Ok(Page {
            data: questions.into_iter().map(QuestionResponse::from).collect(),
            next_cursor: next_cursor.as_ref().map(cursor::encode),
        })
    }

    async fn create_answer(
        &self,
        request: CreateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError> {
        let answer = self
            .question_repo
            .create_answer(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
                request.author_id,
                &request.body,
            )
            .await?
            .or_not_found("question", request.get_question_path())?;

        Ok(answer.into())
    }

    async fn update_answer(
        &self,
        request: UpdateAnswerRequest,
    ) -> Result<AnswerResponse, QuestionError> {
        let answer = self
            .question_repo
            .update_answer(request.id, &request.body)
            .await?
            .or_not_found("answer", request.id)?;

        Ok(answer.into())
    }

    async fn create_question_comment(
        &self,
        request: CreateQuestionCommentRequest,
    ) -> Result<CommentResponse, QuestionError> {
        let comment = self
            .question_repo
            .create_question_comment(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
                request.author_id,
                &request.body,
            )
            .await?
            .or_not_found("question", request.get_question_path())?;

        Ok(comment.into())
    }

    async fn create_answer_comment(
        &self,
        request: CreateAnswerCommentRequest,
    ) -> Result<CommentResponse, QuestionError> {
        let comment = self
            .question_repo
            .create_comment(request.answer_id, request.author_id, &request.body)
            .await?;

        Ok(comment.into())
    }

    async fn update_comment(
        &self,
        request: UpdateCommentRequest,
    ) -> Result<CommentResponse, QuestionError> {
        let comment = self
            .question_repo
            .update_comment(request.id, &request.body)
            .await?
            .or_not_found("comment", request.id)?;

        Ok(comment.into())
    }

    async fn vote_question(
        &self,
        request: VoteQuestionRequest,
    ) -> Result<VoteResponse, QuestionError> {
        let question_id = self
            .question_repo
            .get_question_id(
                request.owner.as_ref(),
                request.repo.as_ref(),
                request.number,
            )
            .await?
            .or_not_found("question", request.get_question_path())?;

        let result = self
            .question_repo
            .vote(
                request.user_id,
                question_id,
                VoteTarget::Question,
                request.value,
            )
            .await?;

        Ok(result.into())
    }

    async fn vote_answer(&self, request: VoteAnswerRequest) -> Result<VoteResponse, QuestionError> {
        let result = self
            .question_repo
            .vote(
                request.user_id,
                request.answer_id,
                VoteTarget::Answer,
                request.value,
            )
            .await?;

        Ok(result.into())
    }

    async fn vote_comment(
        &self,
        request: VoteCommentRequest,
    ) -> Result<VoteResponse, QuestionError> {
        let result = self
            .question_repo
            .vote(
                request.user_id,
                request.comment_id,
                VoteTarget::Comment,
                request.value,
            )
            .await?;

        Ok(result.into())
    }
}
