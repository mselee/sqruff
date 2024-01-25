use super::base::{apply_fixes, Segment};
use super::fix::SourceFix;
use crate::core::parser::markers::PositionMarker;
use crate::helpers::Boxed;

#[derive(Hash, Debug, Clone, Default, PartialEq)]
pub struct KeywordSegment {
    raw: String,
    uuid: uuid::Uuid,
    position_marker: PositionMarker,
}

impl KeywordSegment {
    pub fn new(raw: String, position_marker: PositionMarker) -> Self {
        Self { raw, uuid: uuid::Uuid::new_v4(), position_marker }
    }
}

impl Segment for KeywordSegment {
    fn new(&self, _segments: Vec<Box<dyn Segment>>) -> Box<dyn Segment> {
        KeywordSegment::new(self.raw.clone(), self.position_marker.clone()).boxed()
    }

    fn get_segments(&self) -> Vec<Box<dyn Segment>> {
        Vec::new()
    }

    fn get_raw_segments(&self) -> Vec<Box<dyn Segment>> {
        vec![self.clone().boxed()]
    }

    fn get_raw(&self) -> Option<String> {
        self.raw.clone().into()
    }

    fn get_type(&self) -> &'static str {
        "kw"
    }

    fn is_code(&self) -> bool {
        true
    }

    fn is_comment(&self) -> bool {
        todo!()
    }

    fn is_whitespace(&self) -> bool {
        todo!()
    }

    fn get_position_marker(&self) -> Option<PositionMarker> {
        self.position_marker.clone().into()
    }

    fn set_position_marker(&mut self, _position_marker: Option<PositionMarker>) {
        todo!()
    }

    fn get_uuid(&self) -> Option<uuid::Uuid> {
        self.uuid.into()
    }

    fn apply_fixes(
        &self,
        dialect: crate::core::dialects::base::Dialect,
        fixes: std::collections::HashMap<uuid::Uuid, super::fix::AnchorEditInfo>,
    ) -> (Box<dyn Segment>, Vec<Box<dyn Segment>>, Vec<Box<dyn Segment>>, bool) {
        apply_fixes(self, dialect, fixes)
    }

    fn get_source_fixes(&self) -> Vec<SourceFix> {
        Vec::new()
    }

    fn edit(
        &self,
        _raw: Option<String>,
        _source_fixes: Option<Vec<SourceFix>>,
    ) -> Box<dyn Segment> {
        todo!()
    }

    fn class_types(&self) -> std::collections::HashSet<String> {
        ["keyword", "word"].map(ToOwned::to_owned).into_iter().collect()
    }
}
