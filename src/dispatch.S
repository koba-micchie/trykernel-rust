
.align 2
.thumb_func
.global dispatch_entry

dispatch_entry:
  movs    r0, #1
  msr     primask, r0  // disable interrupt
  ldr     r1, =disp_running
  str     r0, [r1]     // disp_running = 1

  // save context of executing task to stack 
  push    {r4-r7}
  mov     r0, r8
  mov     r1, r9
  mov     r2, r10
  mov     r3, r11
  push    {r0-r3}

  // confirm executing task(cur_task) 
  ldr     r0, =cur_task
  ldr     r1, [r0]     // get pointer to cur_task
  cmp     r1, #0
  // if cur_task == NULL, then jump to disp_010
  beq     disp_010

  // save stack pointer to TCB which cur_task points
  mov     r2, sp
  str     r2, [r1]    

disp_010:
  // confirm schedule task(sche_task)
  ldr     r1, =sche_task
  ldr     r2, [r1]     // get pointer to sche_task
  cmp     r2, #0
  // if sche_task != NULL, then jump to disp_030
  bne     disp_030
  
  // else cur_task = sche_task 
  str     r2, [r0] 

disp_020:
  movs    r3, #0
  msr     primask, r3   // enable interrupt
  movs    r3, #1
  msr     primask, r3   // disable interrupt

  ldr     r2, [r1]      // get pointer to sche_task
  cmp     r2, #0        // if sche_task == NULL, then jump to disp_020(loop)
  beq     disp_020

  // switch task to execute 
disp_030:
  str     r2, [r0]      // cur_task = sche_task 
  ldr     r0, [r2]      // get pointer to TCB which sche_task points 
  mov     sp, r0        // restore stack pointer

  // 7. restore context on stack 
  pop     {r0-r3}
  mov     r11, r3
  mov     r10, r2
  mov     r9, r1
  mov     r8, r0
  pop     {r4-r7}

  ldr     r0, =disp_running //disp_running = 0
  movs    r1, #0
  str     r1, [r0]
  msr     primask, r1       // enable interuupt

  bx      lr

