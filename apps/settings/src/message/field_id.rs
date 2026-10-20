//! 输入框的编号：编辑状态按它存在 `Settings::fields` 里。

/// 输入框：云服务页的三个，短语编辑卡片里的两个，生词本页的搜索框。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldId {
    BaseUrl,
    CandidateColor,
    CandidateBackground,
    CandidateHighlightColor,
    AnnotationColor,
    AnnotationHighlightColor,

    Model,
    ApiKey,
    PhraseCode,
    PhraseText,
    /// 生词本页的搜索框，不存进配置。
    VocabularySearch,
}

impl FieldId {
    /// 存进配置的那几个（短语的两个随卡片一起存）。
    pub const ALL: [Self; 8] = [
        Self::BaseUrl,
        Self::Model,
        Self::ApiKey,
        Self::CandidateColor,
        Self::CandidateBackground,
        Self::CandidateHighlightColor,
        Self::AnnotationColor,
        Self::AnnotationHighlightColor,
    ];

    /// 配置文件 `[predict]` 里的键。
    pub fn key(self) -> &'static str {
        match self {
            Self::BaseUrl => "base_url",
            Self::CandidateColor => "candidate_color",
            Self::CandidateBackground => "candidate_background",
            Self::CandidateHighlightColor => "candidate_highlight_color",
            Self::AnnotationColor => "annotation_color",
            Self::AnnotationHighlightColor => "annotation_highlight_color",

            Self::Model => "model",
            Self::ApiKey => "api_key",
            Self::PhraseCode => "code",
            Self::PhraseText => "text",
            Self::VocabularySearch => "vocabulary-search",
        }
    }
}
