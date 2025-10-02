# Palíndromo

Um **palíndromo** é um número que, quando invertido, permanece igual a ele mesmo.  

**Exemplos:**

- `121` → invertido: `121` ✅ é palíndromo  
- `10` → invertido: `01` ❌ não é palíndromo  
- `12321` → invertido: `12321` ✅ é palíndromo  
- `0` → invertido: `0` ✅ é palíndromo  

---

# Resolução

Existem **duas formas** principais de verificar se um número é um palíndromo:

---

## 1. Força bruta  
**(Fácil de implementar, mas usa mais memória)**

Passos:

1. Converter o número em **string**.  
2. Usar `split()` para transformar a string em **array de caracteres**.  
3. Aplicar `reverse()` no array.  
4. Converter novamente para número e verificar se é igual ao original.  

---

## 2. Abordagem matemática  
**(Mais eficiente, usando operações aritméticas: `%` e `/`)**

Passos:

1. Inicialize `reversed = 0` e `num` como o número original.  
2. Enquanto `num > 0`:

```text
digit = num % 10
reversed = reversed * 10 + digit
num = num / 10
