# Maximum Non Negative Subarray Sum

## Descrição

Este projeto contém a solução de um desafio de **lógica de programação**, no estilo LeetCode, implementada em Rust.

**Problema:**  

Dado um vetor de inteiros `nums`, encontre o **subarray contínuo de maior soma** que **não contenha números negativos** e retorne a soma desse subarray.  

Se todos os números forem negativos, retorne `0`.

**Exemplos:**  

- **Exemplo 1:**  
```text
Input: nums = [1, 2, -3, 4, 5, -1, 2]
Output: 9
Explicação: O subarray [4, 5] é o maior que não contém negativos, soma = 9.

- **Exemplo 2:**  
Input: nums = [-1, -2, -3]
Output: 0
Explicação: Todos os números são negativos.

- **Exemplo 3:**  
Input: nums = [2, 3, 1, -5, 4, 6]
Output: 10
Explicação: O subarray [4, 6] soma 10, que é o maior.