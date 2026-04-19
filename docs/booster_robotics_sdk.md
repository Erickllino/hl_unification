# Booster Robotics SDK

O SDK da Booster Robotics fornece uma interface para controlar os robôs Booster diretamente. Expõe APIs para C++ e Python.

> Necessário **apenas no robô real**. Em simulação, o `booster_deploy` roda com a flag `--sim` e não precisa do SDK.

---

## Ambiente Suportado

| Requisito | Versão |
|-----------|--------|
| OS | Ubuntu 22.04 LTS |
| Arquitetura | aarch64 e x86_64 |
| Compilador | gcc 11.4.0 |

---

## Instalação

O `install_robot.sh` já compila e instala automaticamente. Para instalação manual:

```bash
cd booster_robotics_sdk
sudo ./install.sh   # dependências do sistema
mkdir build && cd build
cmake .. -DBUILD_PYTHON_BINDING=ON
make -j$(nproc)
sudo make install
```

### Verificar instalação

```bash
python3 -c "import booster_robotics_sdk_python; print('SDK OK')"
```

---

## SDK Python

### Instalação via pip (recomendado)

```bash
pip install booster_robotics_sdk_python --user
```

### Build a partir do fonte

```bash
pip3 install pybind11 pybind11-stubgen
# Se pybind11-stubgen não for encontrado após o pip install:
export PATH=/home/<usuario>/.local/bin:$PATH

mkdir build && cd build
cmake .. -DBUILD_PYTHON_BINDING=on
make
sudo make install
```

### Exemplo Python

```bash
python3 python_example/sdk_pybind_b1_exmaple.py 127.0.0.1
```

> Se instalado via pip, os exemplos ficam em `~/.local/lib/python3.10/site-packages/python_example`.

---

## SDK C++

### Build dos exemplos

```bash
mkdir build && cd build
cmake ..
make
```

### Executar exemplos

```bash
# Exemplo de locomotion
./build/b1_loco_example_client 127.0.0.1

# Subscriber de baixo nível
./build/b1_low_level_subscriber

# Qualquer outro exemplo
./build/<nome_do_exemplo> 127.0.0.1
```

---

## Licença

Apache License 2.0. Dependências de terceiros: fastDDS (Apache 2.0), pybind11 (BSD 3-Clause), pybind11-stubgen (MIT).
