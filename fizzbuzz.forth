defword mod 
    over over / floor * -
return

defvar i 1 @i
while !i 101 < do
    !i 15 mod 0 = if
        "fizzbuzz" out
    else !i 3 mod 0 = if*
        "fizz" out
    else !i 5 mod 0 = if*
        "buzz" out
    else 
        !i out
    end
    !i 1 + @i
end
