# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Add a mail account: titles and steps

add-account-title = 메일 계정 추가
add-account-looking = { $address }의 메일 서버를 찾는 중…
add-account-address-intro = 이메일 주소를 입력하세요. Katna가 서버를 찾아 줍니다.
add-account-servers-title = 서버 설정
add-account-servers-intro = Katna가 { $address }의 메일을 읽고 보내는 곳입니다.
add-account-password-title = 비밀번호 입력
add-account-signing-in = 로그인 중…

## Add a mail account: fields

add-account-field-address = 이메일 주소
add-account-incoming = 받는 메일({ $protocol })
add-account-outgoing = 보내는 메일({ $protocol })
add-account-field-server = 서버
add-account-field-port = 포트
add-account-security-none = 없음
add-account-field-username = 사용자 이름
add-account-field-password = 비밀번호
add-account-show-password = 비밀번호 표시
add-account-app-password-hint = { $provider }에서는 웹에서 쓰는 비밀번호가 아니라 앱 비밀번호가 필요합니다. { $provider } 계정의 보안 설정에서 만드세요.
add-account-field-name = 이름(선택 사항)
add-account-name-hint = 메일을 받는 사람에게 표시됩니다.
add-account-servers-pair = { $imap } 및 { $smtp }
add-account-servers-found = { $source ->
    [built-in] 서버: { $servers }, Katna의 제공업체 목록에서 찾음.
    [provider] 서버: { $servers }, 메일 제공업체의 설정에서 찾음.
    [ispdb] 서버: { $servers }, Thunderbird의 제공업체 목록에서 찾음.
    [dns] 서버: { $servers }, 도메인의 DNS 레코드에서 찾음.
   *[other] 서버: { $servers }, 추측한 값입니다. 로그인에 실패하면 확인하세요.
}
add-account-servers-entered = 서버: { $servers }, 입력한 대로.

## Add a mail account: buttons

add-account-servers-button = 서버 설정
add-account-back = 뒤로
add-account-add = 계정 추가
add-account-next = 다음
add-account-cancel = 취소

## Add a mail account: problems

add-account-server-missing = { $kind ->
    [incoming] 받는 메일 서버를 입력하세요.
   *[outgoing] 보내는 메일 서버를 입력하세요.
}
add-account-server-space = { $kind ->
    [incoming] 받는 메일 서버 이름에 공백이 있습니다.
   *[outgoing] 보내는 메일 서버 이름에 공백이 있습니다.
}
add-account-port-invalid = { $kind ->
    [incoming] 받는 메일 포트는 { $min }에서 { $max } 사이의 숫자여야 합니다.
   *[outgoing] 보내는 메일 포트는 { $min }에서 { $max } 사이의 숫자여야 합니다.
}
add-account-address-empty = 이메일 주소를 입력하세요.
add-account-address-invalid = { $example }와(과) 같은 이메일 주소를 입력하세요.
add-account-not-found = Katna가 { $address }의 서버를 찾지 못해 일반적인 이름을 입력했습니다. 메일 제공업체에 확인하세요.
add-account-password-empty = 비밀번호를 입력하세요.
add-account-name-is-password = 이름이 비밀번호와 같습니다. 이 칸에는 다른 사람에게 보일 이름을 입력하세요.
add-account-added = { $address }을(를) 추가했습니다. 메일을 가져오는 중…
add-account-app-password-refused = { $provider }에서 비밀번호를 거부했습니다. 웹에서 쓰는 비밀번호가 아니라 앱 비밀번호가 필요합니다.
add-account-password-refused = 서버에서 비밀번호를 거부했습니다. 확인한 후 다시 시도하세요.

## The account menu (from the account button on the top bar)

add-account-menu-another = 다른 계정 추가
add-account-menu-manage = 계정 관리
