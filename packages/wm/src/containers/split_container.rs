use std::{
  cell::{Ref, RefCell, RefMut},
  collections::VecDeque,
  rc::Rc,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  common::{LengthValue, Rect, TilingDirection},
  impl_common_getters, impl_container_debug,
  impl_position_getters_as_resizable, impl_tiling_direction_getters,
  impl_tiling_size_getters,
};

use super::{
  traits::{
    CommonGetters, PositionGetters, TilingDirectionGetters,
    TilingSizeGetters,
  },
  Container, ContainerDto, DirectionContainer, TilingContainer,
  WindowContainer,
};

#[derive(Clone)]
pub struct SplitContainer(Rc<RefCell<SplitContainerInner>>);

struct SplitContainerInner {
  id: Uuid,
  parent: Option<Container>,
  children: VecDeque<Container>,
  child_focus_order: VecDeque<Uuid>,
  tiling_size: f32,
  tiling_direction: TilingDirection,
  inner_gap: LengthValue,
}

/// User-friendly representation of a split container.
///
/// Used for IPC and debug logging.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitContainerDto {
  id: Uuid,
  parent: Option<Uuid>,
  children: Vec<ContainerDto>,
  child_focus_order: Vec<Uuid>,
  tiling_size: f32,
  width: i32,
  height: i32,
  x: i32,
  y: i32,
  tiling_direction: TilingDirection,
}

impl SplitContainer {
  pub fn new(
    tiling_direction: TilingDirection,
    inner_gap: LengthValue,
  ) -> Self {
    let split = SplitContainerInner {
      id: Uuid::new_v4(),
      parent: None,
      children: VecDeque::new(),
      child_focus_order: VecDeque::new(),
      tiling_size: 1.0,
      tiling_direction,
      inner_gap,
    };

    Self(Rc::new(RefCell::new(split)))
  }

  fn to_dto(&self) -> anyhow::Result<ContainerDto> {
    let rect = self.to_rect()?;
    let children = self
      .children()
      .iter()
      .map(|child| child.to_dto())
      .try_collect()?;

    Ok(ContainerDto::Split(SplitContainerDto {
      id: self.id(),
      parent: self.parent().map(|parent| parent.id()),
      children,
      child_focus_order: self.0.borrow().child_focus_order.clone().into(),
      tiling_size: self.tiling_size(),
      tiling_direction: self.tiling_direction(),
      width: rect.width(),
      height: rect.height(),
      x: rect.x(),
      y: rect.y(),
    }))
  }
}

impl_container_debug!(SplitContainer);
impl_common_getters!(SplitContainer);
impl_tiling_size_getters!(SplitContainer);
impl_tiling_direction_getters!(SplitContainer);
impl_position_getters_as_resizable!(SplitContainer);
