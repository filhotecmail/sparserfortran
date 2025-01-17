subroutine dividir(a, b, resultado)
    real, intent(in) :: a, b
    real, intent(out) :: resultado
    if (b /= 0.0) then
      resultado = a / b
    else
      print *, "Error: Division by zero!"
      resultado = 0.0
    end if
  end subroutine dividir