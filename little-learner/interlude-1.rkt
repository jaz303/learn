#lang racket

(require malt)
(define T tensor)
(define L tlen)
(define R tref)

(T 1 2 3)

(+ (tensor 2) (tensor 7))
(+ (tensor 1 2 3) (tensor 4 5 6))

(+ (tensor (tensor 4 6 7) (tensor 2 0 1)) (tensor (tensor 1 2 2) (tensor 6 3 1)))

(+ 4 (tensor 1 2 3))


(define (sum-1 t)
  (define (sum-1-acc t i a)
    (cond
      ((zero? i) (+ a (R t 0)))
      (else (sum-1-acc t (sub1 i) (+ a (R t i))))))
  (sum-1-acc t (sub1 (L t)) 0.0))


(sum
 (tensor (tensor (tensor 1 2) (tensor 3 4)) (tensor (tensor 5 6) (tensor 7 8))))

; For a tensor t of rank n, the rank of (sum t) is n - 1