# OEIS TUI - Traducciones al Español

# Application
app-title = OEIS TUI
app-subtitle = Enciclopedia en Línea de Secuencias de Enteros

# Greeting Screen
greeting-title = Bienvenido a OEIS TUI
greeting-line1 = Una hermosa interfaz de terminal para explorar secuencias de enteros
greeting-line2 = Presiona 'i' o '/' para comenzar a buscar
greeting-line3 = Presiona 'r' para una secuencia aleatoria
greeting-line4 = Presiona 'w' para modo webcam
greeting-line5 = Presiona 'Ctrl+H' para ayuda
greeting-copyright = © Fundación OEIS Inc. - Todos los datos de secuencias son propiedad de OEIS
greeting-version = Versión 0.1.0

# Bienvenida / estados vacíos
welcome-title = Bienvenido a OEIS TUI
welcome-subtitle = La Enciclopedia en Línea de Secuencias de Enteros (TUI no oficial)
welcome-prompt = Ingresa una secuencia, palabra o número A para empezar.
welcome-search-label = Buscar en OEIS
welcome-enter-hint = Enter para buscar
welcome-esc-hint = Esc para cerrar
welcome-hero-subtitle = Encuentra secuencias conocidas, descubre referencias y explora relaciones.
welcome-hero-tips = Prueba: 1,2,3,4,5,6  •  keyword:prime  •  id:A000045
welcome-hero-search-hint = Pulsa 'i' o '/' en cualquier momento para buscar.
search-empty-title = Aún no hay resultados
search-tips-title = Consejos de búsqueda:
search-tip-terms = • Ingresa términos de secuencia: 1,1,2,3,5,8,13
search-tip-anumber = • Busca por número A: id:A000045
search-tip-keyword = • Busca por palabra clave: fibonacci
search-tip-prefixes = • Usa prefijos: keyword:nice author:Sloane
search-start-hint = Pulsa 'i' o '/' para comenzar a buscar
search-recently-viewed = Visto recientemente
search-history-empty = Aún no hay historial
search-bookmarks-title = Marcadores
search-bookmarks-empty = Aún no hay marcadores. Presione 'b' en la vista detallada para marcar secuencias.
search-bookmarks-loading = Cargando...
search-bookmarks-notes = Notas
search-results-title = Resultados
# Search Screen
search-title = Buscar en OEIS
search-input-label = Búsqueda
search-input-placeholder = Ingresa términos de la secuencia, número A, o palabra clave...
search-status-results = { $count ->
    [0] No se encontraron resultados
    [one] 1 resultado encontrado
    *[other] { $count } resultados encontrados
}
search-status-page = Página { $current } de { $total }
search-status-loading = Buscando...
search-status-fetching = Por favor espere mientras obtenemos resultados de OEIS
search-status-error = Error: { $message }
search-no-results = No se encontraron resultados
search-result-one = 1 resultado encontrado
search-result-many = { $count } resultados encontrados
search-result-many-plus = { $count }+ resultados encontrados
search-table-anumber = Número A
search-table-name = Nombre
search-table-data = Vista Previa de Datos
search-table-views = Vistas
search-block-results = Resultados
search-block-preview = Vista Previa
search-block-details = Detalles
search-preview-empty = No hay vista previa disponible
search-invalid-tab = Pestaña inválida
search-view-count = { $count ->
    [one] 1 vista
    *[other] { $count } vistas
}
search-help = i,/ Buscar | ↑↓ Navegar | ←→ Página | Enter Ver | p Vista previa | r Aleatorio | w Webcam | s Ajustes | Ctrl+H Ayuda | q Salir
search-help-search = Buscar
search-help-navigate = Navegar
search-help-page = Página
search-help-view = Ver
search-help-preview = Vista Previa
search-help-bookmarks = Marcadores
search-help-random = Aleatorio
search-help-webcam = Webcam
search-help-settings = Ajustes
search-help-help = Ayuda
search-help-quit = Salir
search-help-click = Seleccionar
search-help-click-x2 = Abrir
search-help-scroll = Mover

# Detail View
detail-tab-overview = Resumen
detail-tab-formulas = Fórmulas
detail-tab-code = Código
detail-tab-references = Referencias
detail-tab-crossrefs = Referencias cruzadas
detail-tab-metadata = Metadatos
detail-tab-graph = Gráfico
detail-tab-export = Exportar
detail-offset = Desplazamiento
detail-keywords = Palabras clave
detail-author = Autor
detail-created = Creado
detail-modified = Última modificación
detail-comments = Comentarios
detail-data = Datos de la Secuencia
detail-formulas = Fórmulas
detail-examples = Ejemplos
detail-maple = Código Maple
detail-mathematica = Código Mathematica
detail-programs = Otros Programas
detail-references = Referencias
detail-links = Enlaces
detail-crossrefs = Referencias cruzadas
detail-extensions = Extensiones
detail-no-data = No hay datos disponibles
detail-help = Tab Cambiar | ↑↓ Desplazar | g Gráfico | e Exportar | o Navegador | b Marcador | Esc Volver
detail-help-next-link = Siguiente enlace
detail-help-prev-link = Enlace anterior
detail-help-switch-tab = Cambiar pestaña
detail-help-follow-link = Seguir enlace
detail-help-scroll = Desplazar
detail-help-graph = Gráfico
detail-help-export = Exportar
detail-help-browser = Abrir en navegador
detail-help-bookmark = Marcador
detail-bookmarked = Marcado
detail-not-bookmarked = No marcado
detail-help-bfile = Obtener B-file
detail-help-more = Más
detail-help-modal-title = Vista Detallada - Atajos de Teclado
detail-bfile-available = Datos extendidos disponibles
detail-bfile-fetch = Presione 'f' para obtener B-file
detail-bfile-loading = Cargando B-file...
detail-bfile-loaded = ✓ Cargados {$count} términos
detail-bfile-error = B-file no disponible
detail-bfile-not-found = B-file no encontrado para esta secuencia

# Graph View
graph-title = Vista de Gráfico
graph-line = Gráfico de Líneas
graph-scatter = Diagrama de Dispersión
graph-log = Diagrama de Dispersión Logarítmico
graph-pin = Gráfico de Pines
graph-no-data = No hay datos numéricos para graficar
graph-no-positive = No hay valores positivos para escala logarítmica
graph-current = Actual
graph-help = 1 Línea | 2 Dispersión | 3 Log | 4 Pines | Esc Volver

# Export Screen
export-title = Exportar Secuencia
export-format = Seleccionar Formato
export-json = JSON
export-json-desc = Datos completos de la secuencia con metadatos
export-csv = CSV
export-csv-desc = Valores de la secuencia en formato separado por comas
export-txt = TXT
export-txt-desc = Formato de texto plano legible
export-markdown = Markdown
export-markdown-desc = Documentación formateada
export-preview = Vista Previa
export-no-sequence = No hay secuencia para exportar
export-success = Exportado al portapapeles exitosamente
export-file-success = Guardado en archivo: { $path }
export-error = Fallo al exportar: { $message }
export-help = ↑↓ Seleccionar | 1-5 Selección rápida | Enter Portapapeles | Ctrl+S Guardar | Esc Cancelar
export-bfile = B-file
export-bfile-desc = Datos extendidos de secuencia (pares índice valor)
export-bfile-not-loaded = B-file no cargado - presiona 'f' en vista detallada
export-select-format = Seleccionar Formato
export-cancel = Cancelar

# Etiquetas de Contenido de Exportación
export-label-offset = Desplazamiento
export-label-keywords = Palabras clave
export-label-data = Datos
export-label-author = Autor
export-label-created = Creado
export-label-modified = Última modificación
export-label-references = Referencias
export-label-revision = Revisión

# Encabezados de Sección de Exportación
export-section-sequence-data = Datos de Secuencia
export-section-metadata = Metadatos
export-section-comments = Comentarios
export-section-formulas = Fórmulas
export-section-examples = Ejemplos
export-section-code = Código
export-section-references = Referencias
export-section-links = Enlaces
export-section-crossrefs = Referencias cruzadas

# Encabezados de Subsección de Exportación
export-subsection-maple = Maple
export-subsection-mathematica = Mathematica
export-subsection-programs = Otros Programas

# Específico de Formato de Exportación
export-csv-header = Número-A,Nombre,Valores
export-markdown-source = Fuente
export-markdown-oeis-credit = Datos de la Enciclopedia En Línea de Secuencias de Enteros (OEIS)

# Webcam Mode
webcam-title = Webcam OEIS - Explorador de Secuencias
webcam-category = Categoría
webcam-category-all = Todas las Secuencias
webcam-category-all-desc = Explorar todas las secuencias OEIS
webcam-category-best = Mejores Secuencias
webcam-category-best-desc = Secuencias interesantes y notables (palabra clave:nice)
webcam-category-needing = Necesitan Términos
webcam-category-needing-desc = Secuencias que solicitan más términos (palabra clave:more)
webcam-category-recent = Adiciones Recientes
webcam-category-recent-desc = Secuencias añadidas recientemente (palabra clave:new)
webcam-interval = Intervalo de Actualización
webcam-interval-manual = Manual
webcam-interval-manual-desc = Presiona Espacio para avanzar
webcam-interval-5s = 5 segundos
webcam-interval-5s-desc = Actualización automática cada 5s
webcam-interval-10s = 10 segundos
webcam-interval-10s-desc = Actualización automática cada 10s
webcam-interval-20s = 20 segundos
webcam-interval-20s-desc = Actualización automática cada 20s
webcam-interval-30s = 30 segundos
webcam-interval-30s-desc = Actualización automática cada 30s
webcam-interval-1m = 1 minuto
webcam-interval-1m-desc = Actualización automática cada 60s
webcam-current-sequence = Secuencia Actual
webcam-no-sequence = No hay secuencia cargada
webcam-load-first = Presiona Espacio o Enter para cargar la primera secuencia
webcam-refresh-in = Próxima actualización en { $seconds } segundos...
webcam-more-comments = ... y { $count } comentarios más
webcam-help = Espacio/Enter Siguiente | ←→ Categoría | ↑↓ Intervalo | 0-5 Rápido | d Detalle | Esc Volver

# Settings Screen
settings-title = Configuración
settings-language = Idioma
settings-language-desc = Seleccionar idioma de la interfaz
settings-theme = Tema
settings-theme-desc = Esquema de colores (próximamente)
settings-cache = Caché
settings-cache-desc = Administrar caché local
settings-cache-clear = Limpiar Caché
settings-cache-size = Tamaño del caché: { $size }
settings-help = ↑↓ Navegar | Enter Seleccionar | Esc Volver

# About Screen
about-title = Acerca de OEIS TUI
about-version = Versión
about-author = Creado por
about-license = Licencia
about-built-with = Construido con
about-links = Enlaces
about-repository = Repositorio
about-oeis-link = Sitio web de OEIS
about-disclaimer = Este es un cliente no oficial y no está afiliado ni respaldado por The OEIS Foundation Inc.

# Help Screen
help-title = Ayuda - Atajos de Teclado
help-global = Controles Globales
help-global-quit = Salir de la aplicación
help-global-help = Mostrar/ocultar ayuda
help-global-back = Volver / Cancelar
help-search = Pantalla de Búsqueda
help-search-input = Comenzar búsqueda
help-search-navigate = Navegar resultados
help-search-page = Página anterior/siguiente
help-search-view = Ver secuencia seleccionada
help-search-random = Secuencia aleatoria
help-search-preview = Alternar panel de vista previa
help-search-preview-tabs = Cambiar pestaña de vista previa
help-search-mouse-select = Clic para seleccionar resultado
help-search-mouse-open = Doble clic para abrir resultado
help-search-mouse-scroll = Desplazar rueda para mover vista previa/resultados
help-search-webcam = Modo webcam
help-detail = Vista Detallada
help-detail-links = Ciclar enlace resaltado
help-detail-tabs = Cambiar pestañas
help-detail-open-link = Abrir enlace resaltado
help-detail-scroll = Desplazar contenido
help-detail-scroll-fast = Desplazar más rápido
help-detail-graph = Ver gráfico
help-detail-export = Exportar secuencia
help-detail-browser = Abrir en navegador
help-detail-bookmark = Alternar marcador
help-graph = Vista de Gráfico
help-graph-types = Cambiar tipo de gráfico
help-export = Pantalla de Exportación
help-export-select = Seleccionar formato
help-export-quick = Selección rápida de formato
help-export-clipboard = Exportar al portapapeles
help-export-file = Guardar en archivo
help-webcam = Modo Webcam
help-webcam-next = Cargar siguiente secuencia
help-webcam-category = Cambiar categoría
help-webcam-interval = Cambiar intervalo de actualización
help-webcam-quick = Selección rápida de intervalo
help-webcam-detail = Ir a vista detallada

# Common
common-loading = Cargando...
common-error = Error
common-success = Éxito
common-cancel = Cancelar
common-ok = OK
common-yes = Sí
common-no = No
common-back = Volver
common-next = Siguiente
common-previous = Anterior
common-page = Página
common-of = de

# Errors
error-network = Error de red: No se puede conectar a OEIS
error-api = Error de API: { $message }
error-parse = Error de análisis: Formato de datos inválido
error-cache = Error de caché: { $message }
error-clipboard = Error del portapapeles: { $message }
error-file = Error de archivo: { $message }
error-unknown = Ocurrió un error desconocido
detail-no-sequence = No hay secuencia cargada
detail-block-sequence = Secuencia
detail-block-details = Detalles
detail-section-data = Datos
detail-section-comments = Comentarios
detail-section-examples = Ejemplos
graph-help-line = Línea
graph-help-scatter = Dispersión
graph-help-log = Log
graph-help-pin = Pines
graph-help-back = Volver a vista detallada
webcam-sequence-offset = Desplazamiento
webcam-sequence-keywords = Palabras clave
webcam-sequence-data-title = Datos de Secuencia
webcam-sequence-comments-title = Comentarios
webcam-help-next = Siguiente
webcam-help-category = Categoría
webcam-help-interval = Intervalo
webcam-help-quick = Rápido
webcam-help-detail = Detalles
webcam-help-back = Volver
settings-block-themes = Temas
settings-block-animation = Animación de Bienvenida
settings-help-switch = Cambiar Sección
settings-help-navigate = Navegar
settings-help-apply = Aplicar
settings-help-cycle-theme = Cambiar Tema
settings-help-back = Volver
