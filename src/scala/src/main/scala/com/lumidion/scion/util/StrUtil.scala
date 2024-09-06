package com.lumidion.scion.util

import java.nio.charset.StandardCharsets

object StrUtil:
  private val unsafeCharsRegex = "[^\\w@%+=:,./-]".r
  private val sandboxPathRegex = "/.*/pants-sandbox-[a-zA-Z0-9]+/".r
  private val colorsRegex = "\\x1b\\[(K|.*?m)".r
  private val newLineRegex = "(\\r\\n|\\r|\\n)".r

  type BinaryOrText = Array[Byte] | String

  def ensureBinary(input: BinaryOrText): Array[Byte] =
    input match
      case binary: Array[Byte] => binary
      case str: String => str.getBytes("utf8")

  def ensureText(input: BinaryOrText): String =
    input match
      case binary: Array[Byte] => new String(binary, StandardCharsets.UTF_8)
      case str: String => str

  def safeShlexSplit(input: BinaryOrText): List[String] = ??? //TODO: find out if this is needed or remove. No obvious replacement for shlex found

  def shellQuote(str: String): String =
    if (str.isEmpty)
      "''"
    else if (unsafeCharsRegex.matches(str))
      str
    else
      "'" + str.replace("'", "'\"'\"'") + "'"

  def safeShLexJoin(argList: Iterable[String]): String =
    argList.map(shellQuote).mkString(" ")

  private def pluralizeString(str: String): String =
    if (str.endsWith("s"))
      str + "es"
    else if (str.endsWith("y"))
      str.substring(0, str.length - 2) + "ies"
    else str + "s"

  def pluralize(count: Int, itemType: String, includeCount: Boolean = true): String =
    val pluralizedItem = if (count == 1) then itemType else pluralizeString(itemType)
    if (includeCount)
      s"$count, $pluralizedItem"
    else
      pluralizedItem

  def commaSeparatedList(items: Iterable[String]): String =
    val itemList = items.toList
    if (items.isEmpty)
      ""
    else if (itemList.length == 1)
      itemList.head
    else if (itemList.length == 2)
      s"${itemList.head} and ${itemList(1)}"
    else
      s"${itemList.take(itemList.length - 1).mkString(", ")}, and ${itemList.last}"

  def stripPrefix(str: String, prefix: String): String =
    if (str.startsWith(prefix))
      str.substring(prefix.length, str.length - 1)
    else str

  def stripV2ChrootPath(input: BinaryOrText): String =
    val text = ensureText(input)
    sandboxPathRegex.replaceFirstIn(text, "")

  class Simplifier(shouldStripChrootPath: Boolean = true, shouldStripFormatting: Boolean = false) {

    def simplify(input: BinaryOrText): String =
      val chroot = input match
        case str: String => str
        case binary: Array[Byte] =>
          if (shouldStripChrootPath)
            stripV2ChrootPath(input)
          else
            ensureText(input)

      if (shouldStripFormatting)
        colorsRegex.replaceFirstIn(chroot, "")
      else chroot
  }

  //TODO: port text wrapper
  def hardWrap(str: String, indent: Int = 0, width: Int = 96): List[String] = ???

  def bulletList(elements: Iterable[String], maxElements: Option[Int] = None): String =
    if (elements.isEmpty)
      ""
    else
      val elementsList = elements.toList
      val finalList = maxElements
        .map { maximum =>
        if (elementsList.length > maximum)
          elementsList.take(maximum - 1) :+ s"... and ${elementsList.length - maximum + 1} more"
        else elementsList
      }.getOrElse(elementsList)

      val separator = "\n  * "
      s"  * ${finalList.mkString(separator)}"

  def firstParagraph(str: String): String =
    val lines = str.split(newLineRegex.toString())
    val firstBlankLineIndex = lines.zipWithIndex.collectFirst { (line, index) =>
      if (line.trim == "")
        Some(index)
      else None
    }
      .flatten

    firstBlankLineIndex.fold(lines){ index =>
      lines.take(index)
    }
      .mkString.mkString(" ")

  def softWrap(text: String): String = ???