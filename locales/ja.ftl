# OEIS TUI - 日本語翻訳

# Application
app-title = OEIS TUI
app-subtitle = オンライン整数列大辞典

# Greeting Screen
greeting-title = OEIS TUIへようこそ
greeting-line1 = 整数列を探索するための美しいターミナルインターフェース
greeting-line2 = 検索を開始するには 'i' または '/' を押してください
greeting-line3 = ランダムな数列には 'r' を押してください
greeting-line4 = ウェブカムモードには 'w' を押してください
greeting-line5 = ヘルプには 'Ctrl+H' を押してください
greeting-copyright = © OEIS Foundation Inc. - すべての数列データはOEISの財産です
greeting-version = バージョン 0.1.0

# ウェルカム / 空の状態
welcome-title = OEIS TUI へようこそ
welcome-subtitle = オンライン整数列大辞典 (非公式 TUI)
welcome-prompt = 数列、単語、または A 番号を入力してください。
welcome-search-label = OEIS を検索
welcome-enter-hint = Enter で検索
welcome-esc-hint = Esc で閉じる
welcome-hero-subtitle = 既知の整数列を見つけ、参考文献を調べ、関係性を探ります。
welcome-hero-tips = 例: 1,2,3,4,5,6  •  keyword:prime  •  id:A000045
welcome-hero-search-hint = いつでも 'i' または '/' を押して検索に移動できます。
search-empty-title = まだ検索結果がありません
search-tips-title = 検索のヒント:
search-tip-terms = • 数列の項を入力: 1,1,2,3,5,8,13
search-tip-anumber = • A 番号で検索: id:A000045
search-tip-keyword = • キーワードで検索: fibonacci
search-tip-prefixes = • 接頭辞を使用: keyword:nice author:Sloane
search-start-hint = 'i' または '/' を押して検索を始める
search-recently-viewed = 最近見たもの
search-history-empty = 履歴はまだありません
search-bookmarks-title = ブックマーク
search-bookmarks-empty = まだブックマークがありません。詳細ビューで 'b' を押して数列をブックマークしてください。
search-bookmarks-loading = 読み込み中...
search-bookmarks-notes = メモ
search-results-title = 結果
# Search Screen
search-title = OEIS検索
search-input-label = 検索
search-input-placeholder = 数列の項（例：1,2,3,5,8,13）、A番号、またはキーワードを入力...
search-status-results = { $count ->
    [0] 結果が見つかりませんでした
    *[other] { $count } 件の結果が見つかりました
}
search-status-page = ページ { $current } / { $total }
search-status-loading = 検索中...
search-status-error = エラー: { $message }
search-table-anumber = A番号
search-table-name = 名前
search-table-data = データプレビュー
search-table-views = 閲覧数
search-help = i,/ 検索 | ↑↓ ナビゲート | ←→ ページ | Enter 表示 | p プレビュー | r ランダム | w ウェブカム | s 設定 | Ctrl+H ヘルプ | q 終了

# Detail View
detail-tab-overview = 概要
detail-tab-formulas = 数式
detail-tab-code = コード
detail-tab-references = 参考文献
detail-tab-crossrefs = 相互参照
detail-tab-metadata = メタデータ
detail-tab-graph = グラフ
detail-tab-export = エクスポート
detail-offset = オフセット
detail-keywords = キーワード
detail-author = 著者
detail-created = 作成日
detail-modified = 最終更新
detail-comments = コメント
detail-data = 数列データ
detail-formulas = 数式
detail-examples = 例
detail-maple = Mapleコード
detail-mathematica = Mathematicaコード
detail-programs = その他のプログラム
detail-references = 参考文献
detail-links = リンク
detail-crossrefs = 相互参照
detail-extensions = 拡張
detail-no-data = データがありません
detail-help = Tab 切替 | ↑↓ スクロール | g グラフ | e エクスポート | o ブラウザ | b ブックマーク | Esc 戻る
detail-help-next-link = 次のリンク
detail-help-prev-link = 前のリンク
detail-help-switch-tab = タブ切替
detail-help-follow-link = リンクを開く
detail-help-scroll = スクロール
detail-help-graph = グラフ
detail-help-export = エクスポート
detail-help-browser = ブラウザで開く
detail-help-bookmark = ブックマーク
detail-bookmarked = ブックマーク済み
detail-not-bookmarked = 未ブックマーク
detail-help-bfile = B-fileを取得
detail-help-more = 詳細
detail-help-modal-title = 詳細ビュー - キーボードショートカット
detail-bfile-available = 拡張データが利用可能
detail-bfile-fetch = 'f' を押してB-fileを取得
detail-bfile-loading = B-fileを読み込み中...
detail-bfile-loaded = ✓ {$count} 項を読み込みました
detail-bfile-error = B-fileは利用できません
detail-bfile-not-found = この数列のB-fileが見つかりません

# Graph View
graph-title = グラフビュー
graph-line = 折れ線グラフ
graph-scatter = 散布図
graph-log = 対数散布図
graph-pin = ピングラフ
graph-no-data = プロットする数値データがありません
graph-no-positive = 対数スケールの正の値がありません
graph-current = 現在
graph-help = 1 折れ線 | 2 散布図 | 3 対数 | 4 ピン | Esc 戻る

# Export Screen
export-title = 数列をエクスポート
export-format = フォーマットを選択
export-json = JSON
export-json-desc = すべてのメタデータを含む完全な数列データ
export-csv = CSV
export-csv-desc = カンマ区切り形式の数列値
export-txt = TXT
export-txt-desc = 人間が読めるプレーンテキスト形式
export-markdown = Markdown
export-markdown-desc = フォーマットされたドキュメント
export-preview = プレビュー
export-no-sequence = エクスポートする数列がありません
export-success = クリップボードにエクスポートしました
export-file-success = ファイルに保存しました: { $path }
export-error = エクスポートに失敗しました: { $message }
export-help = ↑↓ 選択 | 1-5 クイック選択 | Enter クリップボード | Ctrl+S 保存 | Esc キャンセル
export-bfile = B-file
export-bfile-desc = 拡張数列データ（インデックス 値 ペア）
export-bfile-not-loaded = B-fileが読み込まれていません - 詳細ビューで'f'を押してください
export-select-format = フォーマットを選択
export-cancel = キャンセル

# エクスポートコンテンツラベル
export-label-offset = オフセット
export-label-keywords = キーワード
export-label-data = データ
export-label-author = 著者
export-label-created = 作成日
export-label-modified = 最終更新
export-label-references = 参考文献
export-label-revision = リビジョン

# エクスポートセクションヘッダー
export-section-sequence-data = 数列データ
export-section-metadata = メタデータ
export-section-comments = コメント
export-section-formulas = 数式
export-section-examples = 例
export-section-code = コード
export-section-references = 参考文献
export-section-links = リンク
export-section-crossrefs = 相互参照

# エクスポートサブセクションヘッダー
export-subsection-maple = Maple
export-subsection-mathematica = Mathematica
export-subsection-programs = その他のプログラム

# エクスポートフォーマット固有
export-csv-header = A番号,名前,値
export-markdown-source = ソース
export-markdown-oeis-credit = オンライン整数列大辞典（OEIS）のデータ

# Webcam Mode
webcam-title = OEIS ウェブカム - 数列ブラウザ
webcam-category = カテゴリ
webcam-category-all = すべての数列
webcam-category-all-desc = すべてのOEIS数列を閲覧
webcam-category-best = ベスト数列
webcam-category-best-desc = 興味深く注目すべき数列（キーワード:nice）
webcam-category-needing = 項が必要
webcam-category-needing-desc = より多くの項を要求している数列（キーワード:more）
webcam-category-recent = 最近の追加
webcam-category-recent-desc = 最近追加された数列（キーワード:new）
webcam-interval = 更新間隔
webcam-interval-manual = 手動
webcam-interval-manual-desc = スペースキーで進む
webcam-interval-5s = 5秒
webcam-interval-5s-desc = 5秒ごとに自動更新
webcam-interval-10s = 10秒
webcam-interval-10s-desc = 10秒ごとに自動更新
webcam-interval-20s = 20秒
webcam-interval-20s-desc = 20秒ごとに自動更新
webcam-interval-30s = 30秒
webcam-interval-30s-desc = 30秒ごとに自動更新
webcam-interval-1m = 1分
webcam-interval-1m-desc = 60秒ごとに自動更新
webcam-current-sequence = 現在の数列
webcam-no-sequence = 数列が読み込まれていません
webcam-load-first = スペースキーまたはEnterキーを押して最初の数列を読み込む
webcam-refresh-in = 次の更新まで { $seconds } 秒...
webcam-more-comments = ... および { $count } 件のコメント
webcam-help = Space/Enter 次へ | ←→ カテゴリ | ↑↓ 間隔 | 0-5 クイック | d 詳細 | Esc 戻る

# Settings Screen
settings-title = 設定
settings-language = 言語
settings-language-desc = インターフェース言語を選択
settings-theme = テーマ
settings-theme-desc = 配色（近日公開）
settings-cache = キャッシュ
settings-cache-desc = ローカルキャッシュを管理
settings-cache-clear = キャッシュをクリア
settings-cache-size = キャッシュサイズ: { $size }
settings-help = ↑↓ ナビゲート | Enter 選択 | Esc 戻る

# About Screen
about-title = OEIS TUIについて
about-version = バージョン
about-author = 作成者
about-license = ライセンス
about-built-with = 技術スタック
about-links = リンク
about-repository = リポジトリ
about-oeis-link = OEISウェブサイト
about-disclaimer = これは非公式クライアントであり、The OEIS Foundation Inc.とは関係なく、承認されていません。

# Help Screen
help-title = ヘルプ - キーボードショートカット
help-global = グローバルコントロール
help-global-quit = アプリケーションを終了
help-global-help = ヘルプの表示/非表示
help-global-back = 戻る / キャンセル
help-search = 検索画面
help-search-input = 検索を開始
help-search-navigate = 結果をナビゲート
help-search-page = 前/次のページ
help-search-view = 選択した数列を表示
help-search-random = ランダム数列
help-search-preview = プレビュー パネルの切り替え
help-search-preview-tabs = プレビュータブを切り替え
help-search-mouse-select = クリックで結果を選択
help-search-mouse-open = ダブルクリックで結果を開く
help-search-mouse-scroll = ホイールでプレビュー/結果をスクロール
help-search-webcam = ウェブカムモード
help-detail = 詳細ビュー
help-detail-links = ハイライトされたリンクを巡回
help-detail-tabs = タブを切り替え
help-detail-open-link = ハイライトされたリンクを開く
help-detail-scroll = コンテンツをスクロール
help-detail-scroll-fast = 高速スクロール
help-detail-graph = グラフを表示
help-detail-export = 数列をエクスポート
help-detail-browser = ブラウザで開く
help-detail-bookmark = ブックマークを切り替え
help-graph = グラフビュー
help-graph-types = グラフタイプを切り替え
help-export = エクスポート画面
help-export-select = フォーマットを選択
help-export-quick = クイックフォーマット選択
help-export-clipboard = クリップボードにエクスポート
help-export-file = ファイルに保存
help-webcam = ウェブカムモード
help-webcam-next = 次の数列を読み込む
help-webcam-category = カテゴリを切り替え
help-webcam-interval = 更新間隔を変更
help-webcam-quick = クイック間隔選択
help-webcam-detail = 詳細ビューにジャンプ

# Common
common-loading = 読み込み中...
common-error = エラー
common-success = 成功
common-cancel = キャンセル
common-ok = OK
common-yes = はい
common-no = いいえ
common-back = 戻る
common-next = 次へ
common-previous = 前へ
common-page = ページ
common-of = /

# Errors
error-network = ネットワークエラー: OEISに接続できません
error-api = APIエラー: { $message }
error-parse = 解析エラー: 無効なデータ形式
error-cache = キャッシュエラー: { $message }
error-clipboard = クリップボードエラー: { $message }
error-file = ファイルエラー: { $message }
error-unknown = 不明なエラーが発生しました
search-status-fetching = OEISから結果を取得しています。お待ちください
search-no-results = 結果が見つかりませんでした
search-result-one = 1件の結果が見つかりました
search-result-many = { $count }件の結果が見つかりました
search-result-many-plus = { $count }+件の結果が見つかりました
search-block-results = 結果
search-block-preview = プレビュー
search-block-details = 詳細
search-preview-empty = プレビューはありません
search-invalid-tab = 無効なタブ
search-view-count = { $count ->
    [one] 1回の閲覧
    *[other] { $count }回の閲覧
}
search-help-search = 検索
search-help-navigate = ナビゲート
search-help-page = ページ
search-help-view = 表示
search-help-preview = プレビュー
search-help-bookmarks = ブックマーク
search-help-random = ランダム
search-help-webcam = ウェブカム
search-help-settings = 設定
search-help-help = ヘルプ
search-help-quit = 終了
search-help-click = 選択
search-help-click-x2 = 開く
search-help-scroll = 移動
detail-no-sequence = シーケンスが読み込まれていません
detail-block-sequence = シーケンス
detail-block-details = 詳細
detail-section-data = データ
detail-section-comments = コメント
detail-section-examples = 例
graph-help-line = ライン
graph-help-scatter = 散布図
graph-help-log = 対数
graph-help-pin = ピン
graph-help-back = 詳細ビューに戻る
webcam-sequence-offset = オフセット
webcam-sequence-keywords = キーワード
webcam-sequence-data-title = シーケンスデータ
webcam-sequence-comments-title = コメント
webcam-help-next = 次へ
webcam-help-category = カテゴリ
webcam-help-interval = 間隔
webcam-help-quick = クイック
webcam-help-detail = 詳細表示
webcam-help-back = 戻る
settings-block-themes = テーマ
settings-block-animation = ウェルカムアニメーション
settings-help-switch = セクション切替
settings-help-navigate = ナビゲート
settings-help-apply = 適用
settings-help-cycle-theme = テーマ切替
settings-help-back = 戻る
