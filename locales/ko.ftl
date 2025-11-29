# OEIS TUI - 한국어 번역

# Application
app-title = OEIS TUI
app-subtitle = 온라인 정수 수열 백과사전

# Greeting Screen
greeting-title = OEIS TUI에 오신 것을 환영합니다
greeting-line1 = 정수 수열을 탐색하기 위한 아름다운 터미널 인터페이스
greeting-line2 = 검색을 시작하려면 'i' 또는 '/'를 누르세요
greeting-line3 = 무작위 수열을 보려면 'r'을 누르세요
greeting-line4 = 웹캠 모드를 사용하려면 'w'를 누르세요
greeting-line5 = 도움말을 보려면 'Ctrl+H'를 누르세요
greeting-copyright = © OEIS Foundation Inc. - 모든 수열 데이터는 OEIS의 자산입니다
greeting-version = 버전 0.1.0

# 환영 / 빈 상태
welcome-title = OEIS TUI에 오신 것을 환영합니다
welcome-subtitle = 온라인 정수 수열 백과사전 (비공식 TUI)
welcome-prompt = 수열, 단어 또는 A번호를 입력하세요.
welcome-search-label = OEIS 검색
welcome-enter-hint = Enter로 검색
welcome-esc-hint = Esc로 닫기
welcome-hero-subtitle = 알려진 수열을 찾고, 참고문헌을 살펴보고, 관계를 탐색하세요.
welcome-hero-tips = 예: 1,2,3,4,5,6  •  keyword:prime  •  id:A000045
welcome-hero-search-hint = 언제든 'i' 또는 '/'를 눌러 검색으로 이동하세요.
search-empty-title = 아직 검색 결과가 없습니다
search-tips-title = 검색 팁:
search-tip-terms = • 수열 항 입력: 1,1,2,3,5,8,13
search-tip-anumber = • A번호로 검색: id:A000045
search-tip-keyword = • 키워드로 검색: fibonacci
search-tip-prefixes = • 접두사 사용: keyword:nice author:Sloane
search-start-hint = 'i' 또는 '/'를 눌러 검색을 시작하세요
search-recently-viewed = 최근 본 목록
search-history-empty = 아직 기록이 없습니다
search-bookmarks-title = 북마크
search-bookmarks-empty = 아직 북마크가 없습니다. 상세 보기에서 'b'를 눌러 수열을 북마크하세요.
search-bookmarks-loading = 로딩 중...
search-bookmarks-notes = 메모
search-results-title = 결과
# Search Screen
search-title = OEIS 검색
search-input-label = 검색
search-input-placeholder = 수열 항(예: 1,2,3,5,8,13), A번호 또는 키워드를 입력하세요...
search-status-results = { $count ->
    [0] 결과를 찾을 수 없습니다
    *[other] { $count }개의 결과를 찾았습니다
}
search-status-page = 페이지 { $current } / { $total }
search-status-loading = 검색 중...
search-status-error = 오류: { $message }
search-table-anumber = A번호
search-table-name = 이름
search-table-data = 데이터 미리보기
search-table-views = 조회수
search-help = i,/ 검색 | ↑↓ 탐색 | ←→ 페이지 | Enter 보기 | p 미리보기 | r 무작위 | w 웹캠 | s 설정 | Ctrl+H 도움말 | q 종료

# Detail View
detail-tab-overview = 개요
detail-tab-formulas = 공식
detail-tab-code = 코드
detail-tab-references = 참고문헌
detail-tab-crossrefs = 상호참조
detail-tab-metadata = 메타데이터
detail-tab-graph = 그래프
detail-tab-export = 내보내기
detail-offset = 오프셋
detail-keywords = 키워드
detail-author = 저자
detail-created = 생성일
detail-modified = 마지막 수정
detail-comments = 설명
detail-data = 수열 데이터
detail-formulas = 공식
detail-examples = 예제
detail-maple = Maple 코드
detail-mathematica = Mathematica 코드
detail-programs = 기타 프로그램
detail-references = 참고문헌
detail-links = 링크
detail-crossrefs = 상호참조
detail-extensions = 확장
detail-no-data = 사용 가능한 데이터가 없습니다
detail-help = Tab 전환 | ↑↓ 스크롤 | g 그래프 | e 내보내기 | o 브라우저 | b 북마크 | Esc 뒤로
detail-help-next-link = 다음 링크
detail-help-prev-link = 이전 링크
detail-help-switch-tab = 탭 전환
detail-help-follow-link = 링크 열기
detail-help-scroll = 스크롤
detail-help-graph = 그래프
detail-help-export = 내보내기
detail-help-browser = 브라우저에서 열기
detail-help-bookmark = 북마크
detail-bookmarked = 북마크됨
detail-not-bookmarked = 북마크 안됨
detail-help-bfile = B-file 가져오기
detail-help-more = 더보기
detail-help-modal-title = 상세 보기 - 키보드 단축키
detail-bfile-available = 확장 데이터 사용 가능
detail-bfile-fetch = 'f'를 눌러 B-file을 가져오세요
detail-bfile-loading = B-file 로딩 중...
detail-bfile-loaded = ✓ {$count}개 항목 로드됨
detail-bfile-error = B-file을 사용할 수 없습니다
detail-bfile-not-found = 이 수열에 대한 B-file을 찾을 수 없습니다

# Graph View
graph-title = 그래프 보기
graph-line = 꺾은선 그래프
graph-scatter = 산점도
graph-log = 로그 산점도
graph-pin = 핀 그래프
graph-no-data = 그릴 수치 데이터가 없습니다
graph-no-positive = 로그 스케일에 양수 값이 없습니다
graph-current = 현재
graph-help = 1 꺾은선 | 2 산점도 | 3 로그 | 4 핀 | Esc 뒤로

# Export Screen
export-title = 수열 내보내기
export-format = 형식 선택
export-json = JSON
export-json-desc = 모든 메타데이터가 포함된 전체 수열 데이터
export-csv = CSV
export-csv-desc = 쉼표로 구분된 형식의 수열 값
export-txt = TXT
export-txt-desc = 사람이 읽을 수 있는 일반 텍스트 형식
export-markdown = Markdown
export-markdown-desc = 형식화된 문서
export-preview = 미리보기
export-no-sequence = 내보낼 수열이 없습니다
export-success = 클립보드로 내보내기 성공
export-file-success = 파일에 저장됨: { $path }
export-error = 내보내기 실패: { $message }
export-help = ↑↓ 선택 | 1-5 빠른 선택 | Enter 클립보드 | Ctrl+S 저장 | Esc 취소
export-bfile = B-file
export-bfile-desc = 확장 수열 데이터 (인덱스 값 쌍)
export-bfile-not-loaded = B-file이 로드되지 않음 - 상세 보기에서 'f'를 누르세요
export-select-format = 형식 선택
export-cancel = 취소

# 내보내기 콘텐츠 라벨
export-label-offset = 오프셋
export-label-keywords = 키워드
export-label-data = 데이터
export-label-author = 저자
export-label-created = 생성일
export-label-modified = 마지막 수정
export-label-references = 참고문헌
export-label-revision = 리비전

# 내보내기 섹션 헤더
export-section-sequence-data = 수열 데이터
export-section-metadata = 메타데이터
export-section-comments = 설명
export-section-formulas = 공식
export-section-examples = 예제
export-section-code = 코드
export-section-references = 참고문헌
export-section-links = 링크
export-section-crossrefs = 상호참조

# 내보내기 하위 섹션 헤더
export-subsection-maple = Maple
export-subsection-mathematica = Mathematica
export-subsection-programs = 기타 프로그램

# 내보내기 형식별
export-csv-header = A번호,이름,값
export-markdown-source = 출처
export-markdown-oeis-credit = 온라인 정수 수열 백과사전(OEIS)의 데이터

# Webcam Mode
webcam-title = OEIS 웹캠 - 수열 브라우저
webcam-category = 카테고리
webcam-category-all = 모든 수열
webcam-category-all-desc = 모든 OEIS 수열 탐색
webcam-category-best = 최고의 수열
webcam-category-best-desc = 흥미롭고 주목할 만한 수열 (키워드:nice)
webcam-category-needing = 항이 필요함
webcam-category-needing-desc = 더 많은 항을 요청하는 수열 (키워드:more)
webcam-category-recent = 최근 추가
webcam-category-recent-desc = 최근에 추가된 수열 (키워드:new)
webcam-interval = 새로고침 간격
webcam-interval-manual = 수동
webcam-interval-manual-desc = 스페이스 키를 눌러 진행
webcam-interval-5s = 5초
webcam-interval-5s-desc = 5초마다 자동 새로고침
webcam-interval-10s = 10초
webcam-interval-10s-desc = 10초마다 자동 새로고침
webcam-interval-20s = 20초
webcam-interval-20s-desc = 20초마다 자동 새로고침
webcam-interval-30s = 30초
webcam-interval-30s-desc = 30초마다 자동 새로고침
webcam-interval-1m = 1분
webcam-interval-1m-desc = 60초마다 자동 새로고침
webcam-current-sequence = 현재 수열
webcam-no-sequence = 로드된 수열이 없습니다
webcam-load-first = 스페이스 또는 Enter를 눌러 첫 번째 수열 로드
webcam-refresh-in = 다음 새로고침까지 { $seconds }초...
webcam-more-comments = ... 그리고 { $count }개의 댓글 더
webcam-help = Space/Enter 다음 | ←→ 카테고리 | ↑↓ 간격 | 0-5 빠른 선택 | d 상세 | Esc 뒤로

# Settings Screen
settings-title = 설정
settings-language = 언어
settings-language-desc = 인터페이스 언어 선택
settings-theme = 테마
settings-theme-desc = 색 구성표 (곧 출시)
settings-cache = 캐시
settings-cache-desc = 로컬 캐시 관리
settings-cache-clear = 캐시 지우기
settings-cache-size = 캐시 크기: { $size }
settings-help = ↑↓ 탐색 | Enter 선택 | Esc 뒤로

# About Screen
about-title = OEIS TUI 정보
about-version = 버전
about-author = 제작자
about-license = 라이선스
about-built-with = 기술 스택
about-links = 링크
about-repository = 저장소
about-oeis-link = OEIS 웹사이트
about-disclaimer = 이것은 비공식 클라이언트이며 The OEIS Foundation Inc.와 제휴하거나 승인되지 않았습니다.

# Help Screen
help-title = 도움말 - 키보드 단축키
help-global = 전역 컨트롤
help-global-quit = 애플리케이션 종료
help-global-help = 도움말 표시/숨기기
help-global-back = 뒤로 / 취소
help-search = 검색 화면
help-search-input = 검색 시작
help-search-navigate = 결과 탐색
help-search-page = 이전/다음 페이지
help-search-view = 선택한 수열 보기
help-search-random = 무작위 수열
help-search-preview = 미리보기 패널 토글
help-search-preview-tabs = 미리보기 탭 전환
help-search-mouse-select = 클릭하여 결과 선택
help-search-mouse-open = 더블 클릭하여 결과 열기
help-search-mouse-scroll = 휠로 미리보기/결과 스크롤
help-search-webcam = 웹캠 모드
help-detail = 상세 보기
help-detail-links = 강조된 링크 순환
help-detail-tabs = 탭 전환
help-detail-open-link = 강조된 링크 열기
help-detail-scroll = 콘텐츠 스크롤
help-detail-scroll-fast = 빠른 스크롤
help-detail-graph = 그래프 보기
help-detail-export = 수열 내보내기
help-detail-browser = 브라우저에서 열기
help-detail-bookmark = 북마크 전환
help-graph = 그래프 보기
help-graph-types = 그래프 유형 전환
help-export = 내보내기 화면
help-export-select = 형식 선택
help-export-quick = 빠른 형식 선택
help-export-clipboard = 클립보드로 내보내기
help-export-file = 파일로 저장
help-webcam = 웹캠 모드
help-webcam-next = 다음 수열 로드
help-webcam-category = 카테고리 전환
help-webcam-interval = 새로고침 간격 변경
help-webcam-quick = 빠른 간격 선택
help-webcam-detail = 상세 보기로 이동

# Common
common-loading = 로딩 중...
common-error = 오류
common-success = 성공
common-cancel = 취소
common-ok = 확인
common-yes = 예
common-no = 아니오
common-back = 뒤로
common-next = 다음
common-previous = 이전
common-page = 페이지
common-of = /

# Errors
error-network = 네트워크 오류: OEIS에 연결할 수 없습니다
error-api = API 오류: { $message }
error-parse = 파싱 오류: 잘못된 데이터 형식
error-cache = 캐시 오류: { $message }
error-clipboard = 클립보드 오류: { $message }
error-file = 파일 오류: { $message }
error-unknown = 알 수 없는 오류가 발생했습니다
search-status-fetching = OEIS에서 결과를 가져오는 중입니다. 잠시 기다려주세요
search-no-results = 결과를 찾을 수 없습니다
search-result-one = 1개의 결과를 찾았습니다
search-result-many = { $count }개의 결과를 찾았습니다
search-result-many-plus = { $count }+개의 결과를 찾았습니다
search-block-results = 결과
search-block-preview = 미리보기
search-block-details = 상세정보
search-preview-empty = 미리보기가 없습니다
search-invalid-tab = 유효하지 않은 탭
search-view-count = { $count ->
    [one] 1회 조회
    *[other] { $count }회 조회
}
search-help-search = 검색
search-help-navigate = 탐색
search-help-page = 페이지
search-help-view = 보기
search-help-preview = 미리보기
search-help-bookmarks = 북마크
search-help-random = 임의
search-help-webcam = 웹캠
search-help-settings = 설정
search-help-help = 도움말
search-help-quit = 종료
search-help-click = 선택
search-help-click-x2 = 열기
search-help-scroll = 이동
detail-no-sequence = 수열이 로드되지 않았습니다
detail-block-sequence = 수열
detail-block-details = 상세정보
detail-section-data = 데이터
detail-section-comments = 댓글
detail-section-examples = 예시
graph-help-line = 선
graph-help-scatter = 산점도
graph-help-log = 로그
graph-help-pin = 핀
graph-help-back = 상세 보기로 돌아가기
webcam-sequence-offset = 오프셋
webcam-sequence-keywords = 키워드
webcam-sequence-data-title = 수열 데이터
webcam-sequence-comments-title = 댓글
webcam-help-next = 다음
webcam-help-category = 카테고리
webcam-help-interval = 간격
webcam-help-quick = 빠른 선택
webcam-help-detail = 상세 보기
webcam-help-back = 돌아가기
settings-block-themes = 테마
settings-block-animation = 환영 애니메이션
settings-help-switch = 섹션 전환
settings-help-navigate = 탐색
settings-help-apply = 적용
settings-help-cycle-theme = 테마 순환
settings-help-back = 돌아가기
