; (define fill? color?)
; (struct stroke (color width))
(struct component (builder props transform id))
; (struct component-group (children transform))
(struct anim (duration motion))
(define (dur animation) 
  (if (anim? animation) 
    (anim-duration animation) 
    0.0))

(define (build-object component)
  (if (component? component)
    (let* ((builder (component-builder component))
           (props (component-props component))
           (obj (apply builder props)))
      (apply-transform obj (component-transform component)))
    (error "build-object: not a component" component)))

; Doesn't check if props or transform are equal
(define (component-equal? a b)
  (cond
    [(and (component? a) (component? b))
     (equal? (component-id a) (component-id b))]
    [else #f]))

(define (update-component component updater components)
  (cond 
    [(null? components) 
     (error "update-component: component not found" component)]
    [(component-equal? (car components) component)
     (cons (updater (car components)) (cdr components))]
    [else 
      (cons (car components) (update-component component updater (cdr components)))]))

(define (update-props c updater components)
  (update-component c 
    (lambda (c) 
      (component 
        (component-builder c) 
        (updater (component-props c)) 
        (component-transform c)
        (component-id c)))
    components))

(define (update-transform c updater components)
  (update-component c 
    (lambda (c) 
      (component 
        (component-builder c) 
        (component-props c) 
        (updater (component-transform c))
        (component-id c)))
    components))

(define (add component)
  (lambda (components alpha) (cons component components)))

(define (interp a b alpha)
  (+ a (* (- b a) alpha)))
; (define (interp a b alpha)
;   (+ (* a (- 1.0 alpha)) (* b alpha)))

(define (move component x y)
  (lambda (components alpha) 
    (update-transform 
      component 
      (lambda (t) 
        (transform
          (interp (transform-pos-x t) x alpha)
          (interp (transform-pos-y t) y alpha)
          (transform-rot t)
          (transform-scale t)
          (transform-anchor-x t)
          (transform-anchor-y t)))
      components)))

(define (seq-aux total-duration animations components alpha)
  (if (or (null? animations) (< alpha 0.0))
    components
    (let* ((normalized-alpha (/ (dur (car animations)) total-duration))
          (adjusted-alpha (min (/ alpha normalized-alpha) 1.0))
          (new-components (play (car animations) components adjusted-alpha))
          (new-alpha (- alpha normalized-alpha)))
      (seq-aux total-duration (cdr animations) new-components new-alpha))))

(define (seq animations)
  (let ((total-duration (apply + (map dur animations))))
    (anim total-duration (lambda (components alpha)
                          (seq-aux total-duration animations components alpha)))))

(define (circle x y r fill stroke)
         (component (lambda (r fill stroke)
                      (object-model (draw-circle r) fill stroke))
                    (list r fill stroke)
                    (translation x y)
                    (id)))

(define (line x1 y1 x2 y2 stroke)
         (component (lambda (x1 y1 x2 y2 stroke)
                      (object-model (draw-line x1 y1 x2 y2) '() stroke))
                    (list x1 y1 x2 y2 stroke)
                    (transform-identity)
                    (id)))

(define c1 (circle -2.0 0.0 1.0 (color 255 0 0 255) '()))
(define c2 (circle 0.0 0.0 1.0 (color 0 255 0 255) '()))
(define line (line -2.0 0.0 2.0 0.0 (stroke 0.1 (color 0 0 255 255))))

(define scene (seq (list (add c2)
                         (add c1)
                         (add line)
                         (anim 0.5 (move c1 2.0 -1.0))
                         (anim 0.5 (move c1 0.0 2.0)))))

(define (play animation components alpha)
  (if (anim? animation)
    (let ((motion (anim-motion animation)))
      (motion components alpha))
    (animation components alpha)))

(define (build-object-tree component-tree)
  (map build-object component-tree))
;
(define (main time)
  (object-tree
    (build-object-tree (play scene '() time))))

(define length (dur scene))
