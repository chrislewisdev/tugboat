SECTION "Core subroutines", ROM0

; Sets bc bytes starting from hl to 0
tgZeroMemory::
  .untilAllBytesAreZeroed
    ld [hl], $00
    inc hl
    dec bc
    ld a, b
    or c
  jr nz, .untilAllBytesAreZeroed
  ret

; Copies bc bytes from de to hl
tgCopyMemory::
  .untilAllDataIsCopied
    ld a, [de]
    ld [hli], a
    inc de
    dec bc
    ld a, b
    or c
  jr nz, .untilAllDataIsCopied
  ret

; Divides a by b, storing result in c
tgDivideAB::
  ld c, 0
  .untilDivisionComplete
    sub a, b
    ret c
    inc c
  jr .untilDivisionComplete

; Multiplies register a by register b
tgMultiplyAB::
    dec b
    ret z
    add a
    jr tgMultiplyAB

