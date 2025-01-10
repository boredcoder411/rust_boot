.section .boot, "awx"
.global _start
.code16

_start:
  xor ax, ax
  mov ds, ax
  mov es, ax
  mov ss, ax
  mov fs, ax
  mov gs, ax

  cld

  mov sp, 0x7c00

enable_a20:
  in al, 0x92
  test al, 2
  jnz enable_a20_after
  or al, 2
  and al, 0xfe
  out 0x92, al
enable_a20_after:

check_int13h:
  push 'y'
  mov ah, 0x41
  mov bx, 0x55aa
  int 0x13
  jnc .int13h_ok
  call fail
.int13h_ok:
  pop ax

call_rust:
  push dx
  call first_stage
  push 'x'
  call fail

spin:
  hlt
  jmp spin
