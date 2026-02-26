# 1. Konfiguracja ścieżek
# Po kompilacji za pomocą: cargo build --release --target x86_64-pc-windows-gnu
# Plik DLL znajduje się w poniższej lokalizacji:
dll_path <- "src/rust/target/x86_64-pc-windows-gnu/release/graphidw.dll"

# 2. Ładowanie biblioteki DLL
if (!file.exists(dll_path)) {
  stop(paste("Nie znaleziono pliku DLL w:", dll_path, "\nUpewnij się, że kompilacja zakończyła się sukcesem."))
}

# Wyładowanie jeśli już istnieje (pozwala na przeładowanie po re-kompilacji)
if (is.loaded("wrap__graph_idw_rust")) {
  dyn.unload(dll_path)
}

dyn.load(dll_path)
message("Biblioteka graphidw.dll została pomyślnie załadowana.")

# 3. Definicja wrappera R
graph_idw_rust <- function(from_nodes, values, to_nodes, adj_from, adj_to, adj_weight, p = 2.0, max_dist = 1e10) {
  .Call("wrap__graph_idw_rust",
        as.integer(from_nodes),
        as.numeric(values),
        as.integer(to_nodes),
        as.integer(adj_from),
        as.integer(adj_to),
        as.numeric(adj_weight),
        as.numeric(p),
        as.numeric(max_dist))
}

# 4. Poprawiony Test (Graf Bezkierunkowy symulowany w danych)
# Oryginalne krawędzie: 1 -> 2 (10), 2 -> 3 (10)
adj_from_orig <- c(1, 2)
adj_to_orig   <- c(2, 3)
adj_w_orig    <- c(10, 10)

# Dublowanie krawędzi, aby algorytm mógł "wracać" (bezkierunkowość)
adj_from   <- c(adj_from_orig, adj_to_orig)
adj_to     <- c(adj_to_orig, adj_from_orig)
adj_weight <- c(adj_w_orig, adj_w_orig)

cat("\nUruchamiam test IDW z symulacją grafu bezkierunkowego...\n")

wynik <- graph_idw_rust(
  from_nodes = c(1),           # dane tylko w węźle 1
  values     = c(100),         # wartość 100
  to_nodes   = c(1, 2, 3),     # liczymy dla wszystkich
  adj_from   = adj_from,       # krawędzie w obie strony
  adj_to     = adj_to,
  adj_weight = adj_weight,
  p          = 2.0,
  max_dist   = 100
)

cat("Wyniki dla węzłów 1, 2, 3:\n")
print(wynik)
