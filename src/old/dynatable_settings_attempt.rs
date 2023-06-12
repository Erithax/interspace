

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoStageShown {
    Shown,
    Hidden
}

#[derive(Debug, Clone)]
pub enum PseudoStageFocussedType {
    Isolated,
    Combined,
}

#[derive(Debug, Clone)]
pub enum SelectedStageFocus {
    NonFocus,
    Focus{
        focustype: PseudoStageFocussedType,
        pre: PseudoStageShown,
        post: PseudoStageShown,
    },
}

#[derive(Debug, Clone)]
pub enum StageInfo {
    Normal{
        hidden_bts: Vec<BlockType>,
    },
    Selected{
        focus: SelectedStageFocus,
    }
}


pub struct ColumnSituation {
    tt: DynaTreeType,
    columns: Vec<(Stage, StageInfo)>,
}

pub struct StagePrim {
    stage: Stage,
    shown: bool,
    pseudo_stage_count: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PseudoPrimType {
    Normal,
    Prefocus,
    Focus,
    Postfocus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PseudoPrim {
    stage: Stage,
    shown: bool,
    typ: PseudoPrimType,
}

impl ColumnSituation {
    pub fn assert_valid(&self) {
        for s in Stage::iter_reals() {
            assert!(self.columns.iter().any(|(my_s, _)| *my_s == s));
        }
        for (my_s, _) in self.columns.iter() {
            assert!(Stage::iter_reals().any(|s| s == *my_s));
        }
        
        // one selected stage
        assert!(self.columns.iter().filter(|(_, better_stage)| match better_stage {StageInfo::Selected{..}=>true,_=>false}).count() == 1);

        // stages strictly increase
        assert!(self.columns.iter().enumerate().skip(1).all(|(i, (s, bs))| self.columns[i-1].0 < *s));

        // BetterStage::Normal.hidden_bts are all bt in stage
        assert!(self.columns.iter().all(
            |(s, bs)| match bs {
                StageInfo::Selected {..} => true,
                StageInfo::Normal{hidden_bts} => {hidden_bts.iter().all(|bt| bt.stage() == *s)}
            })
        );

        // when dynatree is Hourglass, selected stage focus must be focus
        assert!(
            {match self.tt {
                DynaTreeType::Hourglass => {
                    match self.get_selected_focus() {
                        SelectedStageFocus::Focus {..} => true,
                        _ => false,
                    }
                },
                _ => true
            }}
        )
    }

    pub fn new_with(tt: DynaTreeType, select: Stage, focus: SelectedStageFocus) -> Self {
        assert!({
            match tt {
                DynaTreeType::Hourglass => {
                    match focus {
                        SelectedStageFocus::Focus {..} => true,
                        _ => false,
                    }
                },
                _ => false,
            }
        });

        let mut cols = Vec::new();
        for s in Stage::iter_reals() {
            if s != select {
                cols.push((s, StageInfo::Normal{hidden_bts: vec![]}))
            } else {
                cols.push((s, StageInfo::Selected{focus: focus.clone()}))
            }
        }
        return Self{
            tt: tt,
            columns: cols,
        }
    }

    pub fn set_stage_info_of(&mut self, s: Stage, si: StageInfo) {
        for (i, (my_stage, my_si)) in self.columns.iter_mut().enumerate() {
            if *my_stage == s {
                *my_si =  si.clone();
            }
        }
    }

    pub fn get_stage_info(&self, s: Stage) -> StageInfo {
        return self.columns.iter().find(|(s, si)| s == s).unwrap().clone().1
    }

    pub fn get_selected_stage(&self) -> Stage {
        return self.columns.iter().find(|(s, si)| match si {StageInfo::Selected {..}=>true,_=>false}).unwrap().0
    }

    pub fn get_selected_stage_info(&self) -> StageInfo {
        return self.columns.iter().find(|(s, si)| match si {StageInfo::Selected {..}=>true,_=>false}).unwrap().clone().1
    }

    pub fn get_selected_focus(&self) -> SelectedStageFocus {
        for (s, bs) in self.columns.iter() {
            match bs {
                StageInfo::Selected { focus } => {return focus.clone()},
                _ => {},
            }
        }
        panic!("no");
    }

    pub fn select_with_prev_focus(&mut self, s: Stage) {
        self.set_stage_info_of(s, StageInfo::Selected { focus: self.get_selected_focus()});
        // todo deselect
    }

    pub fn select_with_prev_focus_substages_shown(&mut self, s: Stage) {
        let prev_focus: SelectedStageFocus = self.get_selected_focus();
        match prev_focus {
            SelectedStageFocus::NonFocus => {
                self.set_stage_info_of(s, StageInfo::Selected { focus: prev_focus});
            },
            SelectedStageFocus::Focus{focustype, ..} => {
                self.set_stage_info_of(
                    s, 
                    StageInfo::Selected {
                        focus: SelectedStageFocus::Focus { 
                            focustype: focustype,
                            pre: PseudoStageShown::Shown, 
                            post: PseudoStageShown::Shown 
                        }
                    }
                )
            }
        }
    }

    pub fn set_focus(&mut self, new_focus: SelectedStageFocus) {
        for (s, si) in self.columns.iter_mut() {
            match si {
                StageInfo::Selected {ref mut focus } => {
                    *focus = new_focus;
                    break
                },
                _ => {}
            }
        }
    }

    pub fn set_tree_type(&mut self, tt: DynaTreeType) {
        match tt {
            DynaTreeType::Hourglass => {
                self.set_focus(SelectedStageFocus::Focus{
                    focustype: PseudoStageFocussedType::Combined,
                    pre: PseudoStageShown::Shown,
                    post: PseudoStageShown::Shown,
                });
                self.tt = tt;
            },
            _ => {
                self.tt = tt;
            }
        }
    }

    pub fn is_stage_shown(&self, s: Stage) -> bool {
        return {
            match self.get_stage_info(s) {
                StageInfo::Selected {..} => true,
                StageInfo::Normal{hidden_bts} => {
                    s.iter_blocktypes().all(|bt| hidden_bts.contains(&bt))
                }
            }
        }
    }

    pub fn is_pseudo_stage_shown(&self, p: PseudoPrimType) -> bool {
        assert!(p != PseudoPrimType::Normal && p != PseudoPrimType::Focus);
        match p {
            PseudoPrimType::Prefocus => {
                return match self.get_selected_focus() {
                    SelectedStageFocus::Focus { focustype, pre, post } => {pre == PseudoStageShown::Shown},
                    _ => panic!("no"),
                }
            },
            PseudoPrimType::Postfocus => {
                return match self.get_selected_focus() {
                    SelectedStageFocus::Focus { focustype, pre, post } => {post == PseudoStageShown::Shown},
                    _ => panic!("no"),
                }
            },
            _ => panic!("no")
        }
    }

    pub fn get_stage_pseudo_count(&self, s: Stage) -> i32 {
        match self.get_stage_info(s) {
            StageInfo::Normal {..} => 1,
            StageInfo::Selected {..} => 3,
        }
    }

    pub fn iter_stages(&self) -> impl Iterator<Item = StagePrim> {
        return self.columns.iter().map(
            |(s, si)|
            StagePrim {
                stage: *s,
                shown: self.is_stage_shown(*s),
                pseudo_stage_count: self.get_stage_pseudo_count(*s),
            }
        ).collect::<Vec<StagePrim>>().into_iter()
    }

    pub fn iter_pseudos(&self) -> impl Iterator<Item = PseudoPrim> {
        let mut res = vec![];
        for (s, si) in self.columns.iter() {
            match si {
                StageInfo::Normal{..} => { 
                    res.push(PseudoPrim{
                        stage: *s,
                        shown: self.is_stage_shown(*s),
                        typ: PseudoPrimType::Normal,
                    });
                },
                StageInfo::Selected{focus} => {
                    res.push(
                        PseudoPrim {
                            stage: *s,
                            shown: self.is_stage_shown(*s),
                            typ: PseudoPrimType::Prefocus,
                        }
                    );
                    res.push(
                        PseudoPrim {
                            stage: *s,
                            shown: self.is_stage_shown(*s),
                            typ: PseudoPrimType::Focus,
                        }
                    );
                    res.push(
                        PseudoPrim {
                            stage: *s,
                            shown: self.is_stage_shown(*s),
                            typ: PseudoPrimType::Postfocus,
                        }
                    );
                }                
            }
        }
        return res.into_iter()
    }

    pub fn get_tree_type_button_prims(&self) -> Vec<(String, bool, bool, DynaTreeType)> {
        // (text, active, clickable, inaction_value)
        let mut res = vec![];
        for tt in DynaTreeType::iter() {
            res.push((tt.to_string(), tt == self.tt, tt != self.tt, tt));
        }
        return res
    }

    pub fn get_focus_type(&self) -> FocusColumn {
        match self.get_selected_focus() {
            SelectedStageFocus::NonFocus => {FocusColumn::Normal},
            SelectedStageFocus::Focus{focustype, ..} => {
                match focustype {
                    PseudoStageFocussedType::Combined => {FocusColumn::Combined},
                    PseudoStageFocussedType::Isolated => {FocusColumn::Isolated}
                }
            }
        }
    }

    pub fn get_focus_type_button_prims(&self) -> Vec<(String, bool, bool, FocusColumn)> {
        let mut res = vec![];
        for fc in FocusColumn::iter() {
            res.push((fc.to_string(), fc == self.get_focus_type(), self.tt != DynaTreeType::Hourglass, fc));
        }
        return res
    }

    pub fn set_focus_by_button(&mut self, fc: FocusColumn) {
        assert!(fc != FocusColumn::Combined || self.tt != DynaTreeType::Hourglass);
        match fc {
            FocusColumn::Normal => {
                self.set_focus(SelectedStageFocus::NonFocus);
            },
            FocusColumn::Isolated => {
                self.set_focus(SelectedStageFocus::Focus { focustype: PseudoStageFocussedType::Isolated, pre: PseudoStageShown::Shown, post: PseudoStageShown::Shown});
            },
            FocusColumn::Combined => {
                self.set_focus(SelectedStageFocus::Focus { focustype: PseudoStageFocussedType::Combined, pre: PseudoStageShown::Shown, post: PseudoStageShown::Shown});
            }
        }
    }

    pub fn get_pseudo_stage_button_prims(&self) -> Vec<(String, bool, bool, (Stage, PseudoPrimType))> {
        let mut res = vec![];
        for (s, si) in self.columns.iter() {
            match si {
                StageInfo::Normal{..} => {
                    res.push((s.to_string(), !self.is_stage_shown(*s), true, (*s, PseudoPrimType::Normal)));
                },
                StageInfo::Selected { .. } => {
                    res.push(("pre-".to_string() + s.to_string().as_str(), !self.is_pseudo_stage_shown(PseudoPrimType::Prefocus), true, (*s, PseudoPrimType::Prefocus)));
                    res.push((s.to_string(), false, false, (*s, PseudoPrimType::Focus)));
                    res.push(("post-".to_string() + s.to_string().as_str(), !self.is_pseudo_stage_shown(PseudoPrimType::Postfocus), true, (*s, PseudoPrimType::Postfocus)));
                }
            }
        }
        return res
    }

    pub fn set_stage_visibility_by_button(&mut self, p: (Stage, PseudoPrimType)) {
        assert!(p.1 == PseudoPrimType::Normal || p.0 == self.get_selected_stage());
  
    }

}