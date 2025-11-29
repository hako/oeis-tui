# OEIS TUI - Traductions Françaises

# Application
app-title = OEIS TUI
app-subtitle = Encyclopédie en Ligne des Suites Entières

# Greeting Screen
greeting-title = Bienvenue dans OEIS TUI
greeting-line1 = Une belle interface terminal pour explorer les suites entières
greeting-line2 = Appuyez sur 'i' ou '/' pour commencer la recherche
greeting-line3 = Appuyez sur 'r' pour une suite aléatoire
greeting-line4 = Appuyez sur 'w' pour le mode webcam
greeting-line5 = Appuyez sur 'Ctrl+H' pour l'aide
greeting-copyright = © Fondation OEIS Inc. - Toutes les données de suites sont la propriété de l'OEIS
greeting-version = Version 0.1.0

# Accueil / états vides
welcome-title = Bienvenue sur OEIS TUI
welcome-subtitle = L'Encyclopédie en ligne des suites d'entiers (TUI non officiel)
welcome-prompt = Saisissez une suite, un mot ou un numéro A pour commencer.
welcome-search-label = Rechercher sur l’OEIS
welcome-enter-hint = Entrée pour rechercher
welcome-esc-hint = Échap pour fermer
welcome-hero-subtitle = Trouvez des suites connues, découvrez des références et explorez des relations.
welcome-hero-tips = Essayez : 1,2,3,4,5,6  •  keyword:prime  •  id:A000045
welcome-hero-search-hint = Appuyez sur 'i' ou '/' à tout moment pour rechercher.
search-empty-title = Aucun résultat pour l’instant
search-tips-title = Conseils de recherche :
search-tip-terms = • Entrez les termes de la suite : 1,1,2,3,5,8,13
search-tip-anumber = • Cherchez par numéro A : id:A000045
search-tip-keyword = • Cherchez par mot-clé : fibonacci
search-tip-prefixes = • Utilisez les préfixes : keyword:nice author:Sloane
search-start-hint = Appuyez sur 'i' ou '/' pour commencer à chercher
search-recently-viewed = Récemment consulté
search-history-empty = Aucun historique pour le moment
search-bookmarks-title = Favoris
search-bookmarks-empty = Aucun favori pour le moment. Appuyez sur 'b' dans la vue détaillée pour marquer des séquences.
search-bookmarks-loading = Chargement...
search-bookmarks-notes = Notes
search-results-title = Résultats
# Search Screen
search-title = Rechercher dans OEIS
search-input-label = Recherche
search-input-placeholder = Entrez les termes de la suite, numéro A, ou mot-clé...
search-status-results = { $count ->
    [0] Aucun résultat trouvé
    [one] 1 résultat trouvé
    *[other] { $count } résultats trouvés
}
search-status-page = Page { $current } sur { $total }
search-status-loading = Recherche en cours...
search-status-fetching = Veuillez patienter pendant que nous récupérons les résultats de l'OEIS
search-status-error = Erreur: { $message }
search-no-results = Aucun résultat trouvé
search-result-one = 1 résultat trouvé
search-result-many = { $count } résultats trouvés
search-result-many-plus = { $count }+ résultats trouvés
search-table-anumber = Numéro A
search-table-name = Nom
search-table-data = Aperçu des Données
search-table-views = Vues
search-block-results = Résultats
search-block-preview = Aperçu
search-block-details = Détails
search-preview-empty = Aucun aperçu disponible
search-invalid-tab = Onglet invalide
search-view-count = { $count ->
    [one] 1 vue
    *[other] { $count } vues
}
search-help = i,/ Rechercher | ↑↓ Naviguer | ←→ Page | Entrée Voir | p Aperçu | r Aléatoire | w Webcam | s Réglages | Ctrl+H Aide | q Quitter
search-help-search = Rechercher
search-help-navigate = Naviguer
search-help-page = Page
search-help-view = Voir
search-help-preview = Aperçu
search-help-bookmarks = Favoris
search-help-random = Aléatoire
search-help-webcam = Webcam
search-help-settings = Réglages
search-help-help = Aide
search-help-quit = Quitter
search-help-click = Sélectionner
search-help-click-x2 = Ouvrir
search-help-scroll = Déplacer

# Detail View
detail-tab-overview = Vue d'ensemble
detail-tab-formulas = Formules
detail-tab-code = Code
detail-tab-references = Références
detail-tab-crossrefs = Références croisées
detail-tab-metadata = Métadonnées
detail-tab-graph = Graphique
detail-tab-export = Exporter
detail-offset = Décalage
detail-keywords = Mots-clés
detail-author = Auteur
detail-created = Créé le
detail-modified = Dernière modification
detail-comments = Commentaires
detail-data = Données de la Suite
detail-formulas = Formules
detail-examples = Exemples
detail-maple = Code Maple
detail-mathematica = Code Mathematica
detail-programs = Autres Programmes
detail-references = Références
detail-links = Liens
detail-crossrefs = Références croisées
detail-extensions = Extensions
detail-no-data = Aucune donnée disponible
detail-no-sequence = Aucune suite chargée
detail-block-sequence = Suite
detail-block-details = Détails
detail-section-data = Données
detail-section-comments = Commentaires
detail-section-examples = Exemples
detail-help = Tab Changer | ↑↓ Défiler | g Graphique | e Exporter | o Navigateur | b Favori | Échap Retour
detail-help-next-link = Lien suivant
detail-help-prev-link = Lien précédent
detail-help-switch-tab = Changer d'onglet
detail-help-follow-link = Suivre le lien
detail-help-scroll = Défiler
detail-help-graph = Graphique
detail-help-export = Exporter
detail-help-browser = Ouvrir dans le navigateur
detail-help-bookmark = Favori
detail-bookmarked = Marqué en favori
detail-not-bookmarked = Non marqué
detail-help-bfile = Récupérer B-file
detail-help-more = Plus
detail-help-modal-title = Vue Détaillée - Raccourcis Clavier
detail-bfile-available = Données étendues disponibles
detail-bfile-fetch = Appuyez sur 'f' pour récupérer le B-file
detail-bfile-loading = Chargement du B-file...
detail-bfile-loaded = ✓ {$count} termes chargés
detail-bfile-error = B-file non disponible
detail-bfile-not-found = B-file introuvable pour cette suite

# Graph View
graph-title = Vue Graphique
graph-line = Graphique Linéaire
graph-scatter = Nuage de Points
graph-log = Nuage de Points Logarithmique
graph-pin = Graphique à Épingles
graph-no-data = Aucune donnée numérique à tracer
graph-no-positive = Aucune valeur positive pour l'échelle logarithmique
graph-current = Actuel
graph-help = 1 Ligne | 2 Nuage | 3 Log | 4 Épingles | Échap Retour
graph-help-line = Ligne
graph-help-scatter = Nuage
graph-help-log = Log
graph-help-pin = Épingles
graph-help-back = Retour à la vue détaillée

# Export Screen
export-title = Exporter la Suite
export-format = Sélectionner le Format
export-json = JSON
export-json-desc = Données complètes avec métadonnées
export-csv = CSV
export-csv-desc = Valeurs en format séparé par virgules
export-txt = TXT
export-txt-desc = Format texte lisible
export-markdown = Markdown
export-markdown-desc = Documentation formatée
export-preview = Aperçu
export-no-sequence = Aucune suite à exporter
export-success = Exporté dans le presse-papiers
export-file-success = Enregistré dans: { $path }
export-error = Échec de l'export: { $message }
export-help = ↑↓ Sélectionner | 1-5 Sélection rapide | Entrée Presse-papiers | Ctrl+S Sauvegarder | Échap Annuler
export-bfile = B-file
export-bfile-desc = Données de suite étendues (paires indice valeur)
export-bfile-not-loaded = B-file non chargé - appuyez sur 'f' dans la vue détaillée
export-select-format = Sélectionner le Format
export-cancel = Annuler

# Étiquettes de Contenu d'Export
export-label-offset = Décalage
export-label-keywords = Mots-clés
export-label-data = Données
export-label-author = Auteur
export-label-created = Créé le
export-label-modified = Dernière modification
export-label-references = Références
export-label-revision = Révision

# En-têtes de Section d'Export
export-section-sequence-data = Données de la Suite
export-section-metadata = Métadonnées
export-section-comments = Commentaires
export-section-formulas = Formules
export-section-examples = Exemples
export-section-code = Code
export-section-references = Références
export-section-links = Liens
export-section-crossrefs = Références croisées

# En-têtes de Sous-section d'Export
export-subsection-maple = Maple
export-subsection-mathematica = Mathematica
export-subsection-programs = Autres Programmes

# Spécifique au Format d'Export
export-csv-header = Numéro-A,Nom,Valeurs
export-markdown-source = Source
export-markdown-oeis-credit = Données de l'Encyclopédie en Ligne des Suites d'Entiers (OEIS)

# Webcam Mode
webcam-title = Webcam OEIS - Navigateur de Suites
webcam-category = Catégorie
webcam-category-all = Toutes les Suites
webcam-category-all-desc = Parcourir toutes les suites OEIS
webcam-category-best = Meilleures Suites
webcam-category-best-desc = Suites intéressantes et notables (mot-clé:nice)
webcam-category-needing = Termes Manquants
webcam-category-needing-desc = Suites demandant plus de termes (mot-clé:more)
webcam-category-recent = Ajouts Récents
webcam-category-recent-desc = Suites récemment ajoutées (mot-clé:new)
webcam-interval = Intervalle de Rafraîchissement
webcam-interval-manual = Manuel
webcam-interval-manual-desc = Appuyez sur Espace pour avancer
webcam-interval-5s = 5 secondes
webcam-interval-5s-desc = Rafraîchissement automatique toutes les 5s
webcam-interval-10s = 10 secondes
webcam-interval-10s-desc = Rafraîchissement automatique toutes les 10s
webcam-interval-20s = 20 secondes
webcam-interval-20s-desc = Rafraîchissement automatique toutes les 20s
webcam-interval-30s = 30 secondes
webcam-interval-30s-desc = Rafraîchissement automatique toutes les 30s
webcam-interval-1m = 1 minute
webcam-interval-1m-desc = Rafraîchissement automatique toutes les 60s
webcam-current-sequence = Suite Actuelle
webcam-no-sequence = Aucune suite chargée
webcam-load-first = Appuyez sur Espace ou Entrée pour charger la première suite
webcam-refresh-in = Prochain rafraîchissement dans { $seconds } secondes...
webcam-more-comments = ... et { $count } commentaires supplémentaires
webcam-sequence-offset = Décalage
webcam-sequence-keywords = Mots-clés
webcam-sequence-data-title = Données de la Suite
webcam-sequence-comments-title = Commentaires
webcam-help = Espace/Entrée Suivant | ←→ Catégorie | ↑↓ Intervalle | 0-5 Rapide | d Détails | Échap Retour
webcam-help-next = Suivant
webcam-help-category = Catégorie
webcam-help-interval = Intervalle
webcam-help-quick = Rapide
webcam-help-detail = Détails
webcam-help-back = Retour

# Settings Screen
settings-title = Paramètres
settings-language = Langue
settings-language-desc = Sélectionner la langue de l'interface
settings-theme = Thème
settings-theme-desc = Schéma de couleurs (à venir)
settings-cache = Cache
settings-cache-desc = Gérer le cache local
settings-cache-clear = Vider le Cache
settings-cache-size = Taille du cache: { $size }
settings-block-themes = Thèmes
settings-block-animation = Animation de Bienvenue
settings-help = ↑↓ Naviguer | Entrée Sélectionner | Échap Retour
settings-help-switch = Changer de Section
settings-help-navigate = Naviguer
settings-help-apply = Appliquer
settings-help-cycle-theme = Changer de Thème
settings-help-back = Retour

# About Screen
about-title = À propos de OEIS TUI
about-version = Version
about-author = Créé par
about-license = Licence
about-built-with = Construit avec
about-links = Liens
about-repository = Dépôt
about-oeis-link = Site Web OEIS
about-disclaimer = Ceci est un client non officiel et n'est pas affilié ou approuvé par The OEIS Foundation Inc.

# Help Screen
help-title = Aide - Raccourcis Clavier
help-global = Contrôles Globaux
help-global-quit = Quitter l'application
help-global-help = Afficher/masquer l'aide
help-global-back = Retour / Annuler
help-search = Écran de Recherche
help-search-input = Commencer la recherche
help-search-navigate = Naviguer dans les résultats
help-search-page = Page précédente/suivante
help-search-view = Voir la suite sélectionnée
help-search-random = Suite aléatoire
help-search-preview = Activer/désactiver le panneau d'aperçu
help-search-preview-tabs = Changer d'onglet d'aperçu
help-search-mouse-select = Clic pour sélectionner le résultat
help-search-mouse-open = Double clic pour ouvrir le résultat
help-search-mouse-scroll = Molette pour faire défiler aperçu/résultats
help-search-webcam = Mode webcam
help-detail = Vue Détaillée
help-detail-links = Parcourir le lien en surbrillance
help-detail-tabs = Changer d'onglet
help-detail-open-link = Ouvrir le lien en surbrillance
help-detail-scroll = Défiler le contenu
help-detail-scroll-fast = Défiler plus vite
help-detail-graph = Voir le graphique
help-detail-export = Exporter la suite
help-detail-browser = Ouvrir dans le navigateur
help-detail-bookmark = Basculer favori
help-graph = Vue Graphique
help-graph-types = Changer le type de graphique
help-export = Écran d'Export
help-export-select = Sélectionner le format
help-export-quick = Sélection rapide du format
help-export-clipboard = Exporter vers le presse-papiers
help-export-file = Sauvegarder dans un fichier
help-webcam = Mode Webcam
help-webcam-next = Charger la suite suivante
help-webcam-category = Changer de catégorie
help-webcam-interval = Changer l'intervalle de rafraîchissement
help-webcam-quick = Sélection rapide de l'intervalle
help-webcam-detail = Aller à la vue détaillée

# Common
common-loading = Chargement...
common-error = Erreur
common-success = Succès
common-cancel = Annuler
common-ok = OK
common-yes = Oui
common-no = Non
common-back = Retour
common-next = Suivant
common-previous = Précédent
common-page = Page
common-of = sur

# Errors
error-network = Erreur réseau: Impossible de se connecter à OEIS
error-api = Erreur API: { $message }
error-parse = Erreur d'analyse: Format de données invalide
error-cache = Erreur de cache: { $message }
error-clipboard = Erreur de presse-papiers: { $message }
error-file = Erreur de fichier: { $message }
error-unknown = Une erreur inconnue s'est produite
