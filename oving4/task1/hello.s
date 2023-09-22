section .data
  msg: db "Hello World from Trondheim!", 10

section .text

global _start


_start:
  mov rcx, 3

top:
  push rcx


  mov rax, 1
  mov rdi, 2
  mov rsi, msg
  mov rdx, 28
  syscall

  pop rcx
  dec rcx
  jnz top

  mov rax, 60
  mov rdi, 1
  syscall
