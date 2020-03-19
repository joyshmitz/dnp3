package dev.gridio.dnp3.codegen.model.enums

import dev.gridio.dnp3.codegen.model.{EnumModel, EnumValue, Hex}


object EventMode {

  private val comments = List(
    "Describes how a transaction behaves with respect to event generation"
  )

  def apply(): EnumModel = EnumModel("EventMode", comments, EnumModel.UInt8, codes, None, Hex)

  private val codes = List(
    EnumValue("Detect", 0, "Detect events using the specific mechanism for that type"),
    EnumValue("Force", 1, "Force the creation of an event bypassing detection mechanism"),
    EnumValue("Suppress", 2, "Never produce an event regardless of changes"),
    EnumValue("EventOnly", 3, "Send an event directly to the event buffer, bypassing the static value completely")
  )

}


