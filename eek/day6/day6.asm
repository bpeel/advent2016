#include "electron.asm"

        * = $1910

        ;; 4-byte bitmask of questions that the current group has
        ;; potentially asked so far
        GROUP_ALL_ASKED = $70
        ;; 4-byte bitmask of questions that the current person has
        ;; asked so far.
        PERSON_ASKED = $74
        ;; 4-byte counter for the answer
        RESULT = $78
        ;; temporary store for the question number
        QUESTION = $7C
        ;; bit mask to check for this question
        QUESTION_BIT = $7D
        ;; scratch space for printing a number
        SCRATCH = $80

        ldx #<filename
        ldy #>filename
        lda #$40                ; open
        jsr OSFIND

        .(
        cmp #0
        bne gotfile
        brk
        .byt 255, "Error opening file", 0
gotfile:        
        .)

        ;; store the file descriptor in Y so we can use it for OS calls
        tay

        ;; reset the RESULT
        .(
        ldx #3
        lda #0
loop:   sta RESULT, x
        dex
        bpl loop
        .)

        ;; check for EOF
        .(
inputloop:      
        tya
        tax
        lda #$7f
        jsr OSBYTE
        cpx #0
        bne done

        jsr readgroup

        ldx #3
        .(
loop:   lda GROUP_ALL_ASKED, x
bitloop:
        lsr
        bcc noinc

        pha
        lda RESULT
        adc #0
        sta RESULT
        lda RESULT + 1
        adc #0
        sta RESULT + 1
        lda RESULT + 2
        adc #0
        sta RESULT + 2
        lda RESULT + 3
        adc #0
        sta RESULT + 3
        pla
noinc:  bne bitloop
        
        dex
        bpl loop
        .)

        jmp inputloop

done:   
        lda #0
        jsr OSFIND              ; close the file
        jsr printsum
        lda #13
        jmp OSASCI
        .)

readgroup:
        .(
        ;; All questions have potentially been asked so far
        lda #$FF
        ldx #3
        .(
loop:   sta GROUP_ALL_ASKED, x
        dex
        bpl loop
        .)

personloop:
        jsr readperson

        ;; if the person asked no questions, then it’s either a blank line or
        ;; the end of the file. Either way it’s the end of the group
        lda PERSON_ASKED
        ora PERSON_ASKED + 1
        ora PERSON_ASKED + 2
        ora PERSON_ASKED + 3
        bne notend
        rts
notend: 

        ldx #3
        .(
loop:   lda GROUP_ALL_ASKED, x
        and PERSON_ASKED, x
        sta GROUP_ALL_ASKED, x
        dex
        bpl loop
        .)

        jmp personloop
        .)

readperson:
        .(
        ;; No questions have been asked so far
        lda #0
        ldx #3
        .(
loop:   sta PERSON_ASKED, x
        dex
        bpl loop
        .)

loop:   jsr OSBGET
        cmp #$fe                ; is it EOF?
        bcs done
        cmp #10
        beq done

        sec
        sbc #"a"                ; calculate question number
        bcc loop                ; ignore invalid question numbers
        cmp #32
        bcs loop

        sta QUESTION

        ;; calculate question bit
        .(
        and #7
        tax
        lda #0
        sec
bitloop:
        rol
        dex
        bpl bitloop
        sta QUESTION_BIT
        .)

        ;; calculate byte number in x
        lda QUESTION
        lsr
        lsr
        lsr
        tax

        ;; set the question
        lda PERSON_ASKED, x
        ora QUESTION_BIT
        sta PERSON_ASKED, x
        jmp loop

done:   rts
        .)

        ;; routine to print 4-byte number in RESULT. uses SCRATCH as
        ;; scratch space. corrupts number in RESULT
printsum:  
        .(
        ;; generate 10 digits
        ldy #9
loop:   jsr div_by_ten
        ;; remainder in a
        clc
        adc #"0"
        sta SCRATCH, y
        dey
        bpl loop
        .)
        .(
        iny
        ;; skip leading zeros
loop:   lda SCRATCH, y
        cmp #"0"
        bne done
        iny
        cpy #9 ; leave at least one digit
        bcc loop
done:
        .)
        .(
loop:   lda SCRATCH, y
        jsr $ffee
        iny
        cpy #10
        bcc loop
        .)
        rts

        ;; routine to divide 4-byte number in RESULT by 10
div_by_ten:     
        .( 
        ldx #32                 ; 32 bits
        lda #0
loop:   asl RESULT + 0
        rol RESULT + 1
        rol RESULT + 2
        rol RESULT + 3
        rol
        cmp #10
        bcc nobit
        sbc #10
        inc RESULT
nobit:  
        dex
        bne loop
        rts
        .)
        
filename:
        .byt "data", 13
