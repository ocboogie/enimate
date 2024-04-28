; (define fill? color?)
; (struct stroke (color width))
(struct component (builder props transform))
(struct component-group (children transform))

(define (build-object component)
  (if (component? component)
    (let* ((builder (component-builder component))
           (props (component-props component))
           (obj (apply builder props)))
      (apply-transform obj (component-transform component)))
    (error "build-object: not a component" component)))

; Doesn't not check if props or transform are equal, but just ensures that
(define (component-equal? a b)
  (cond
    [(and (component? a) (component? b))
     ((equal? (component-builder a) (component-builder b)))]
    [else #f]))

; (define (update-component-object component object-table)
;   (if (component? component)
;     (let ((builder (component-builder component))
;           (props (component-props component)))
;      (hash-update object-table builder (builder props)))
;     (error "update-component: not a component" component)))
;
; (define (update-component component updater components object-table)
;   (cond
;     [(component? (car components))
;      (if (component-equal? (car components) component)
;        (let ((updated-component (updater (car component))))
;          (cons (cons updated-component (cdr components))
;                (update-component-object updated-component object-table)))
;        (cons component (update-props component prop-updater (cdr components) object-table)))]
;     [(component-group? (car components))
;      (let ((result (update-component component updater (component-group-children (car components)) object-table))
;            (updated-children (car result))
;            (object-table (cdr result))
;            (result (update-component component updater (cdr components) object-table))
;            (updated-components (car result))
;            (object-table (cdr result))
;            (components (cons (component-group updated-children (component-group-children (car components)))
;                              updated-components)))
;        (cons components object-table))]))

; (define (update-props c updater components object-table)
;   (update-component 
;     c 
;     (lambda (c) 
;       (component 
;         (component-builder c) 
;         (updater (component-props c)) 
;         (component-transform c)))
;     components 
;     object-table))

; (define (update-transform c updater components object-table)
;   (update-component 
;     c 
;     (lambda (c) 
;       (component 
;         (component-builder component) 
;         (component-props component) 
;         (updater (component-transform c))))
;     components 
;     object-table))

(define (add component)
  (lambda (components alpha) (cons component components)))

(define (move component x y)
  (lambda (components alpha) components))

(define (seq-aux motions components alpha)
  (if (null? motions)
    components
    (let ((new-components ((car motions) components alpha)))
      (seq-aux (cdr motions) new-components alpha))))

(define (seq motions)
  (lambda (components alpha)
    (seq-aux motions components alpha)))

(define (circle x y r fill stroke)
         (component (lambda (r fill stroke)
                      (object-model (draw-circle r) fill stroke))
                    (list r fill stroke)
                    (transform-translate x y)))

(define c1 (circle -1.0 0.0 1.0 (color 255 0 0 255) '()))
(define c2 (circle 1.0 0.0 1.0 (color 0 255 0 255) '()))

(define scene (seq (list (add c1) (add c2))))
; (define scene (list c1 c2))

; (define (build-object-tree component-tree)
;   (cond 
;     [(component? component-tree)
;      (component-builder component-tree)]
;     [(component-group? component-tree)
;      (map build-object-tree (component-group-children component-tree))]))

(define (play motion components time)
  (motion components time))

(define (build-object-tree component-tree)
  (map build-object component-tree))
;
(define (main time)
  (object-tree
    (build-object-tree (play scene '() time))))

(define length 1.0)
