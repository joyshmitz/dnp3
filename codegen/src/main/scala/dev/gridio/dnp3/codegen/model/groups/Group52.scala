package dev.gridio.dnp3.codegen.model.groups

import dev.gridio.dnp3.codegen.model.FixedSizeField._
import dev.gridio.dnp3.codegen.model.{FixedSize, GroupType, ObjectGroup, OtherGroupType, Variation}

object Group52 extends ObjectGroup {
  def variations: List[Variation] = List(Group52Var1, Group52Var2)

  def group: Byte = 52

  def desc: String = "Time Delay"

  override def groupType: GroupType = OtherGroupType
}

object Group52Var1 extends FixedSize(Group52, 1, "Coarse")(time16)

object Group52Var2 extends FixedSize(Group52, 2, "Fine")(time16)