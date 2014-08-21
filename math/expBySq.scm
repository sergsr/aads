;;; Tail-recursive exponentiation by squaring with bitshifting.
;;; Only nonnegative integers supported (simply returns 1 for negative exponents)
(define fast-power (lambda (b e)
  (if (< e 1)
      1
      (let loop ((m b) (r 1) (te e))
        (cond ((= 0 te)   r)
              ((even? te) (loop (* m m) r       (ash te -1)))
              (else       (loop (* m m) (* m r) (ash te -1))))))))

