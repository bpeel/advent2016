(defun day18-read-operand ()
  (cond ((looking-at " *[0-9]")
         (read (current-buffer)))
        ((looking-at " *(")
         (goto-char (match-end 0))
         (let ((val (day18-read-expr)))
           (unless (looking-at " *)")
             (error "Missing “)”"))
           (goto-char (match-end 0))
           val))
        (t (error "Invalid expression"))))

(defun day18-read-expr ()
  (interactive)

  (let ((res (day18-read-operand)))

    (while (looking-at " *\\([*+]\\)")
      (let ((op (intern (match-string 1)))
            (b (progn (goto-char (match-end 0))
                      (day18-read-operand))))
        (setq res (list op res b))))

    res))

(defun day18-part1 (beg end)
  (interactive "r")

  (save-excursion
    (goto-char beg)
    (let ((sum 0))
      (while (< (point) end)
        (setq sum (+ sum (eval (day18-read-expr))))
        (forward-line))

      (message (format "Part 1: %i" sum)))))
