#lang racket

(require malt)

(define (line x)
  (lambda (ps)
    (+ (last ps) (* (first ps) x))))