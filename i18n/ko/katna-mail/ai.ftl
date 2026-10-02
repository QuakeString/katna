# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

compose-ai-rephrase-tip = 다시 쓰기(Ctrl+J)
compose-ai-tone-clearer = 더 명확하게
compose-ai-tone-shorter = 더 짧게
compose-ai-tone-friendlier = 더 친근하게
compose-ai-tone-formal = 격식 있게
compose-ai-tone-grammar = 문법 교정
compose-ai-tone-longer = 더 길게
compose-ai-custom = 원하는 방식을 입력하세요…
compose-ai-more = 다른 방식
compose-ai-replace = 바꾸기
compose-ai-again = 다시 시도
compose-ai-below = 아래에 추가
compose-ai-copy = 복사
compose-ai-cancel = 취소
compose-ai-rephrase = 다시 쓰기
compose-ai-replaced = 다시 썼습니다
compose-ai-added = 아래에 추가했습니다
compose-ai-copied = 복사했습니다
compose-ai-katna = Katna AI
compose-ai-own = 내 AI 서비스
compose-ai-trial-left = { $service } · { $days ->
   *[other] 무료 이용 { $days }일 남음
}
compose-ai-encrypted = 이 메일은 암호화됩니다. 다시 쓰기를 하면 선택한 텍스트가 암호화되지 않은 상태로 { $service }에 전송됩니다. 그래도 다시 쓸까요?
compose-ai-sign-in = Katna AI를 사용하려면 Katna 계정이 필요합니다. 로그인하거나 내 API 키를 사용하세요.
compose-ai-pay = Katna AI 무료 이용 기간이 끝났습니다. 월 $5에 이용하거나 내 API 키를 사용할 수 있습니다.
compose-ai-too-many = 지금은 요청이 너무 많습니다. 잠시 후 다시 시도하세요.
compose-ai-no-key = 다시 쓰기를 사용하려면 설정에서 { $service } 키를 추가하세요.
compose-ai-bad-key = { $service }에서 키를 받아들이지 않았습니다. 설정에서 확인하세요.
compose-ai-off = 설정에서 AI 글쓰기 도우미가 꺼져 있습니다.
compose-ai-failed = { $service }에 연결할 수 없습니다. 다시 시도하세요.
compose-ai-try-again = 다시 시도
compose-ai-open-settings = 설정 열기
compose-ai-write-reply-tip = 답장 작성(Ctrl+J)
compose-ai-write-note-tip = 메모 작성(Ctrl+J)
compose-ai-rephrase-empty-tip = 다시 쓸 내용을 입력하세요
compose-ai-write-reply = 답장 작성
compose-ai-write-note = 메모 작성
compose-ai-write-from = { $count ->
   *[other] 메일 { $count }개 바탕
}
compose-ai-write-ideas = 대화에서 얻은 아이디어
compose-ai-write-own = 또는 쓸 내용을 직접 입력하세요…
compose-ai-write-short = 짧게
compose-ai-write-longer = 더 길게
compose-ai-write-friendly = 친근하게
compose-ai-write-formal = 격식 있게
compose-ai-write-insert = 삽입
compose-ai-write-back = 다른 아이디어
compose-ai-written = 초안을 추가했습니다
compose-ai-write-encrypted = 이 대화는 암호화되어 있습니다. 답장을 작성하면 대화의 메일이 암호화되지 않은 상태로 { $service }에 전송됩니다. 그래도 작성할까요?
compose-ai-write-anyway = 작성
compose-ai-write-encrypted-off = 이 대화는 암호화되어 있으며, 설정에 따라 암호화된 메일에는 글쓰기 도우미를 사용하지 않습니다.
compose-ai-subject-tip = 제목 다시 쓰기
compose-ai-subject-title = 다른 표현
compose-ai-subject-done = 제목을 바꿨습니다

## Summing up a conversation: the list's right-click menu, the reading
## pane's sparkle, the chat's strip and the card each opens.

summary-summarize = 요약
summary-hide = 요약 숨기기
summary-close = 닫기
summary-fold = 접기
summary-title = 요약
summary-mails = { $count ->
   *[other] 메일 { $count }개
}
summary-of-mails = 메일 { $total }개 중 { $count }개
summary-peek-count = { $mails ->
   *[other] 메일 { $mails }개
} · { $people ->
   *[other] { $people }명
}
summary-catch-up = { $count ->
   *[other] 마지막으로 읽은 후 새 메일 { $count }개
}
summary-strip-newer = { $count ->
   *[other] 그 후 새 메일 { $count }개 · { $gist }
}
summary-add-new = { $count ->
   *[other] 새 메일 { $count }개 추가
}
summary-point-settled = 결정됨
summary-point-money = 금액
summary-point-dates = 날짜
summary-point-next = 다음 단계
summary-point-open = 미결
summary-files = 파일
summary-for-you = 나에게 해당
summary-from-mail = { $name }, { $date }
summary-you = 나
summary-made = { $service } · { $time }
summary-not-read = { $service } · 읽음으로 표시 안 함
summary-copy = 복사
summary-copied = 요약을 복사했습니다
summary-again = 다시 요약
summary-open = 열기
summary-open-tip = 대화 열기
summary-reply = 답장
summary-reply-tip = AI로 답장 작성
summary-reply-to = { $name }님에게 답장
summary-reply-summary = 요약
summary-reply-send = 보내기
summary-reply-open = 열기
summary-asking = { $service }에 요청하는 중…
summary-stop = 중지
summary-cancel = 취소
summary-send = 보내고 요약
summary-ask-short = 확인을 기다리는 중
summary-encrypted = 이 대화는 암호화되어 있습니다. 요약하면 텍스트가 암호화되지 않은 상태로 { $service }에 전송됩니다.
summary-encrypted-off = 이 대화는 암호화되어 있으며, 설정에 따라 암호화된 메일에는 글쓰기 도우미를 사용하지 않습니다.
summary-try-again = 다시 시도
summary-open-settings = 설정 열기
