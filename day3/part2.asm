section .data
    input_path: db 'input.txt',0

section .bss
    filebuf: resb 11000

section .text
    global _start

_start:
    mov rdi,input_path
    mov rsi,0           ; o_rdonly
    mov rax,2           ; open syscall
    syscall             ; rax contains fd

    mov rdi,rax         ; fd in param 1
    mov rsi,filebuf
    mov rdx,11000
    mov rax,0           ; read syscall
    syscall

    mov rbx,0
    mov r14,0           ; total score

    mov r8,0            ; line 1 start

nextgroup:


; find next newline

    mov r9,r8           ; line 2 start

line2adv:
    add r9,1
    cmp byte filebuf[r9],10
    jne line2adv

    add r9,1
    mov r10,r9          ; line 3 start

line3adv:
    add r10,1
    cmp byte filebuf[r10],10
    jne line3adv

    add r10,1

    mov r11,0   ; line 1 offset
    mov r12,0   ; line 2 offset
    mov r13,0   ; line 3 offset



; find duplicates
    
line2find:
    mov bl,filebuf[r8 + r11]
    cmp filebuf[r9 + r12],bl
    je  line3find
    add r12,1
    cmp byte filebuf[r9 + r12],10
    jne line2find
    add r11,1
    mov r12,0
    jmp line2find

line3find:
    cmp filebuf[r10 + r13],bl
    je getscore
    add r13,1
    cmp byte filebuf[r10 + r13],10
    jne line3find
    add r11,1
    mov r12,0
    mov r13,0
    jmp line2find


getscore:
    cmp bl,96
    jg  lowercase

; uppercase
    sub bl,38
    jmp accscore

lowercase:
    sub bl,96

accscore:
    add r14,rbx
    

; find start of next line

    mov r8,r10

nextline:
    add r8,1
    cmp byte filebuf[r8],10
    jne nextline

    add r8,1
    cmp r8,rax
    jne nextgroup


end:

    mov r12,r14

; now print it out
    mov rax,0
    mov eax,10000
    mov edx,0
    mov ebx,10

    mov r11,0   ; pos in stirng

nextdgt:
    mov r9,0
    mov r10,0

incdgt:
    add r9,rax
    cmp r9,r12
    jg bigger
    add r10,1
    jmp incdgt

bigger:
    add r12,rax
    sub r12,r9
    div ebx
    add r10,48
    mov filebuf[r11],r10b
    add r11,1
    cmp rax,0
    jne nextdgt

    mov byte filebuf[r11],10
    add r11,1
    mov rax,1
    mov rdi,1
    mov rsi,filebuf
    mov rdx,r11
    syscall

    mov rdi,r12
    mov rax,60          ; exit syscall
    syscall

