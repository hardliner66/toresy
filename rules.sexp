(((pattern
    (AnyIdentifier . "x")
    (Symbol . "+")
    (Number . "0")
  )
   (replacement (Identifier . "x"))
 )
  ((pattern
     (Number . "0")
     (Symbol . "+")
     (AnyIdentifier . "x")
   )
    (replacement (Identifier . "x"))
  )
  ((pattern
     (Number . "0")
     (Symbol . "+")
     (AnyNumber . "x")
   )
    (replacement (Number . "x"))
  )
  ((pattern
     (AnyNumber . "x")
     (Symbol . "+")
     (Number . "0")
   )
    (replacement (Number . "x"))
  )
  ((pattern
     (Number . "0")
     (Symbol . "+")
     (Number . "0")
   )
    (replacement (Number . "0"))
  )
)