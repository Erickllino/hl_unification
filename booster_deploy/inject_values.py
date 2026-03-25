"""
inject_values.py
----------------
Lança a simulação como subprocess e injeta 3 valores via stdin.
Você escolhe UMA VEZ qual variável incrementar, depois o loop roda sozinho.

Uso:
    python inject_values.py

Durante o loop:
    Ctrl+C → encerra
"""

import subprocess
import sys
import time

# ─── Configuração ──────────────────────────────────────────────────────────────
SIM_COMMAND    = ["python", "scripts/deploy.py", "--task", "t1_walk", "--mujoco"]
INCREMENT      = 0.05
WAIT_SECONDS   = 10
DECIMAL_PLACES = 2
# ───────────────────────────────────────────────────────────────────────────────

def fmt(v: float) -> str:
    rounded = round(v, DECIMAL_PLACES)
    if rounded == int(rounded):
        return str(int(rounded))
    return f"{rounded:.{DECIMAL_PLACES}f}".rstrip('0')

def send_values(proc, v1, v2, v3):
    line = f"{fmt(v1)} {fmt(v2)} {fmt(v3)}\n"
    print(f"  → Enviando: {line.strip()}", flush=True)
    proc.stdin.write(line)
    proc.stdin.flush()

def choose_variable():
    print("╔══════════════════════════════════════╗")
    print("║       inject_values.py               ║")
    print("╠══════════════════════════════════════╣")
    print("║  Qual variável incrementar?          ║")
    print("║  1 → v1                              ║")
    print("║  2 → v2                              ║")
    print("║  3 → v3                              ║")
    print("╠══════════════════════════════════════╣")
    print("║  Direção do incremento?              ║")
    print("║  + → positivo  (+0.05)               ║")
    print("║  - → negativo  (-0.05)               ║")
    print("╚══════════════════════════════════════╝")
    while True:
        choice = input("  Variável (1/2/3): ").strip()
        if choice in ('1', '2', '3'):
            break
        print("  Opção inválida, tente novamente.")

    while True:
        direction = input("  Direção (+/-): ").strip()
        if direction in ('+', '-'):
            break
        print("  Opção inválida, tente novamente.")

    return int(choice), (1 if direction == '+' else -1)

def main():
    target, sign = choose_variable()
    step = round(INCREMENT * sign, 10)
    direction_label = f"+{INCREMENT}" if sign == 1 else f"-{INCREMENT}"
    print(f"\n  Incrementando v{target} em {direction_label} a cada {WAIT_SECONDS}s.\n")

    print("Iniciando simulação...")
    print(f"  Comando: {' '.join(SIM_COMMAND)}\n")

    proc = subprocess.Popen(
        SIM_COMMAND,
        stdin=subprocess.PIPE,
        stdout=sys.stdout,
        stderr=sys.stderr,
        text=True,
        bufsize=1,
    )

    v = [0.0, 0.0, 0.0]

    try:
        while True:
            if proc.poll() is not None:
                print("\nSimulação encerrada.")
                break

            send_values(proc, v[0], v[1], v[2])
            print(f"  Aguardando {WAIT_SECONDS}s...  (Ctrl+C para encerrar)", flush=True)
            time.sleep(WAIT_SECONDS)

            v[target - 1] = round(v[target - 1] + step, 10)

    except KeyboardInterrupt:
        print("\n\nInterrompido (Ctrl+C).")
    finally:
        if proc.poll() is None:
            proc.terminate()
            proc.wait()
        print("Simulação encerrada.")

if __name__ == "__main__":
    main()