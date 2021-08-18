# RSA-Complexidade-Algoritmos

Implementa um algorítmo de RSA que usa miller-rabin para geração dos primos e algorítmo de Euclides estendido para a geranção da chave privada.

### Código

* src/rsa/utils.rs

    Estão os algorítmos de exponenciação modular rápida, inverso modular e Euclides estendido.

* src/rsa/prime_generator.rs

    Algorítmos para geração dos primos.

* src/rsa/key_generator.rs

    Criação das chaves do RSA.

* src/rsa/encoder.rs

    Criptografia e descriptografia do arquivo de entrada.

* src/breaker.rs

    Algorítmo de pollar-rho de fatoração, para quebrar a chave RSA. 

## Execução

* O projeto pode ser executado usando o comando:
    ```shell
    cargo run
    ```

* Para ver mais sobre os comandos possíveis
    ```
    cargo run help
    ```

* Para executar os testes de tempo de quebra da chave:

    ```shell
    cargo run break
    ```

* Para executar os testes:
    ```shell
    cargo test
    ```

