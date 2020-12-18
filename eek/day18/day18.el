(defun day18-read-operand (&optional part2)
  (cond ((looking-at " *[0-9]")
         (read (current-buffer)))
        ((looking-at " *(")
         (goto-char (match-end 0))
         (let ((val (day18-read-expr part2)))
           (unless (looking-at " *)")
             (error "Missing “)”"))
           (goto-char (match-end 0))
           val))
        (t (error "Invalid expression"))))

(defun day18-read-expr (&optional part2)
  (interactive "P")

  (let ((res (day18-read-operand part2)))
    (while (looking-at " *\\([*+]\\)")
      (let* ((op (intern (match-string 1)))
             (b (progn (goto-char (match-end 0))
                       (if (and part2 (eq op '*))
                           (day18-read-expr part2)
                         (day18-read-operand part2)))))
        (setq res (list op res b))))

    res))

(defun day18 (beg end &optional part2)
  (interactive "r\nP")

  (message "%s" part2)

  (save-excursion
    (goto-char beg)
    (let ((sum 0))
      (while (< (point) end)
        (setq sum (+ sum (eval (day18-read-expr part2))))
        (forward-line))

      (message (format "Part %i: %i" (if part2 2 1) sum)))))
