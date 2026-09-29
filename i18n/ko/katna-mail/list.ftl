# Katna Mail, Korean (한국어).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = 기본
tab-promotions = 프로모션
tab-social = 소셜
tab-updates = 업데이트
tab-forums = 포럼
tab-focused = 중요
tab-other = 기타
tab-inbox = 받은편지함
tab-newsletters = 뉴스레터
tab-notifications = 알림
tab-new = 새 메일 { $count }개
tab-provider-other = Katna에서 분류

## Mail list: toolbar

list-select = 선택
list-refresh = 새로고침
list-checking = 새 메일을 확인하는 중…
list-more = 더보기
list-mark-read = 읽음으로 표시
list-mark-unread = 읽지 않음으로 표시
list-move-to = 이동
list-archive = 보관처리
list-spam = 스팸신고
list-delete = 삭제
list-snooze = 다시 알림
list-unsnooze = 다시 알림 취소
list-newer = 최신
list-older = 이전
list-range = { $first }–{ $last } / { $total }
list-range-about = { $first }–{ $last } / 약 { $total }
list-results = “{ $query }” 검색결과
list-results-corrected = “{ $query }” 검색결과를 표시합니다
list-search-instead = 대신 “{ $query }”(으)로 검색
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = 전체
list-pick-none = 선택 안함
list-pick-read = 읽음
list-pick-unread = 읽지 않음
list-pick-starred = 별표 있음
list-pick-unstarred = 별표 없음

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] 대화 { $count }개가 모두 선택되었습니다.
   *[message] 메일 { $count }개가 모두 선택되었습니다.
}
list-selected-all-in = { $kind ->
    [conversation] { $folder }의 대화 { $count }개가 모두 선택되었습니다.
   *[message] { $folder }의 메일 { $count }개가 모두 선택되었습니다.
}
list-selected-screen = { $kind ->
    [conversation] 이 페이지의 대화 { $count }개가 모두 선택되었습니다.
   *[message] 이 페이지의 메일 { $count }개가 모두 선택되었습니다.
}
list-select-all = { $kind ->
    [conversation] 대화 { $count }개 모두 선택
   *[message] 메일 { $count }개 모두 선택
}
list-select-all-in = { $kind ->
    [conversation] { $folder }의 대화 { $count }개 모두 선택
   *[message] { $folder }의 메일 { $count }개 모두 선택
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] 이 페이지의 읽은 대화 { $count }개가 모두 선택되었습니다.
       *[message] 이 페이지의 읽은 메일 { $count }개가 모두 선택되었습니다.
    }
   *[unread] { $kind ->
        [conversation] 이 페이지의 읽지 않은 대화 { $count }개가 모두 선택되었습니다.
       *[message] 이 페이지의 읽지 않은 메일 { $count }개가 모두 선택되었습니다.
    }
    [starred] { $kind ->
        [conversation] 이 페이지의 별표가 있는 대화 { $count }개가 모두 선택되었습니다.
       *[message] 이 페이지의 별표가 있는 메일 { $count }개가 모두 선택되었습니다.
    }
    [unstarred] { $kind ->
        [conversation] 이 페이지의 별표가 없는 대화 { $count }개가 모두 선택되었습니다.
       *[message] 이 페이지의 별표가 없는 메일 { $count }개가 모두 선택되었습니다.
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] 읽은 대화 { $count }개 모두 선택
       *[message] 읽은 메일 { $count }개 모두 선택
    }
   *[unread] { $kind ->
        [conversation] 읽지 않은 대화 { $count }개 모두 선택
       *[message] 읽지 않은 메일 { $count }개 모두 선택
    }
    [starred] { $kind ->
        [conversation] 별표가 있는 대화 { $count }개 모두 선택
       *[message] 별표가 있는 메일 { $count }개 모두 선택
    }
    [unstarred] { $kind ->
        [conversation] 별표가 없는 대화 { $count }개 모두 선택
       *[message] 별표가 없는 메일 { $count }개 모두 선택
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $folder }의 읽은 대화 { $count }개 모두 선택
       *[message] { $folder }의 읽은 메일 { $count }개 모두 선택
    }
   *[unread] { $kind ->
        [conversation] { $folder }의 읽지 않은 대화 { $count }개 모두 선택
       *[message] { $folder }의 읽지 않은 메일 { $count }개 모두 선택
    }
    [starred] { $kind ->
        [conversation] { $folder }의 별표가 있는 대화 { $count }개 모두 선택
       *[message] { $folder }의 별표가 있는 메일 { $count }개 모두 선택
    }
    [unstarred] { $kind ->
        [conversation] { $folder }의 별표가 없는 대화 { $count }개 모두 선택
       *[message] { $folder }의 별표가 없는 메일 { $count }개 모두 선택
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] 읽은 대화 { $count }개가 모두 선택되었습니다.
       *[message] 읽은 메일 { $count }개가 모두 선택되었습니다.
    }
   *[unread] { $kind ->
        [conversation] 읽지 않은 대화 { $count }개가 모두 선택되었습니다.
       *[message] 읽지 않은 메일 { $count }개가 모두 선택되었습니다.
    }
    [starred] { $kind ->
        [conversation] 별표가 있는 대화 { $count }개가 모두 선택되었습니다.
       *[message] 별표가 있는 메일 { $count }개가 모두 선택되었습니다.
    }
    [unstarred] { $kind ->
        [conversation] 별표가 없는 대화 { $count }개가 모두 선택되었습니다.
       *[message] 별표가 없는 메일 { $count }개가 모두 선택되었습니다.
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $folder }의 읽은 대화 { $count }개가 모두 선택되었습니다.
       *[message] { $folder }의 읽은 메일 { $count }개가 모두 선택되었습니다.
    }
   *[unread] { $kind ->
        [conversation] { $folder }의 읽지 않은 대화 { $count }개가 모두 선택되었습니다.
       *[message] { $folder }의 읽지 않은 메일 { $count }개가 모두 선택되었습니다.
    }
    [starred] { $kind ->
        [conversation] { $folder }의 별표가 있는 대화 { $count }개가 모두 선택되었습니다.
       *[message] { $folder }의 별표가 있는 메일 { $count }개가 모두 선택되었습니다.
    }
    [unstarred] { $kind ->
        [conversation] { $folder }의 별표가 없는 대화 { $count }개가 모두 선택되었습니다.
       *[message] { $folder }의 별표가 없는 메일 { $count }개가 모두 선택되었습니다.
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] 여기에 읽은 대화가 없습니다.
       *[message] 여기에 읽은 메일이 없습니다.
    }
   *[unread] { $kind ->
        [conversation] 여기에 읽지 않은 대화가 없습니다.
       *[message] 여기에 읽지 않은 메일이 없습니다.
    }
    [starred] { $kind ->
        [conversation] 여기에 별표가 있는 대화가 없습니다.
       *[message] 여기에 별표가 있는 메일이 없습니다.
    }
    [unstarred] { $kind ->
        [conversation] 여기에 별표가 없는 대화가 없습니다.
       *[message] 여기에 별표가 없는 메일이 없습니다.
    }
}
list-clear-selection = 선택 해제

## Mail list: empty states

list-empty-search = 검색과 일치하는 메일이 없습니다.
list-empty-tab = { $tab }에 메일이 없습니다.
list-empty-tab-unknown = 이 탭에 메일이 없습니다.
list-empty-folder = { $folder }에 메일이 없습니다.
list-empty-folder-unknown = 이 폴더에 메일이 없습니다.
list-first-sync = 메일을 가져오는 중…
list-first-sync-detail = 메일이 도착하는 대로 여기에 표시됩니다.

## Mail list: lines

row-removed = 이 메일은 삭제되었습니다.
row-starred = 별표 있음
row-not-starred = 별표 없음
row-important = 중요. 클릭하면 중요하지 않음으로 표시합니다.
row-mark-important = 중요 표시
row-pinned = 상단에 고정됨
row-task = 할 일
row-task-open = 할 일 열기: { $title }
row-tracking-none = 추적 중. 아직 열람하지 않음
row-tracking-opened = { $recipients }명 중 { $opened }명이 열람
row-tracking-clicked = { $recipients }명 중 { $opened }명이 열람, { $clicked }명이 링크 클릭
row-pin = 상단에 고정
row-unpin = 고정 해제
row-snoozed-until = { $when }에 다시 알림

## Mail list: More menu and right-click menu

menu-reply = 답장
menu-reply-all = 전체답장
menu-forward = 전달
menu-archive = 보관처리
menu-delete = 삭제
menu-delete-forever = 영구삭제
menu-move-to-inbox = 받은편지함으로 이동
menu-spam = 스팸신고
menu-not-spam = 스팸 아님
menu-mark-read = 읽음으로 표시
menu-mark-unread = 읽지 않음으로 표시
menu-mark-all-read = 모두 읽음으로 표시
menu-star = 별표 추가
menu-unstar = 별표 삭제
menu-important = 중요 표시
menu-not-important = 중요하지 않음으로 표시
menu-pin = 상단에 고정
menu-unpin = 고정 해제
menu-snooze = 다시 알림
menu-unsnooze = 다시 알림 취소
menu-add-to-tasks = 할 일에 추가
menu-schedule-meeting = 회의 예약
menu-start-call = 화상 통화 시작
menu-add-note = 메모 추가
menu-print-all = 모두 인쇄
menu-new-window = 새 창에서 열기
menu-move-to = 이동
# Opens a submenu: Add to Tasks, Add a note, Schedule a meeting and Start a
# video call.
menu-follow-up = 후속 조치
# Opens a submenu of the rarer actions: Report spam, Mark as important and
# Pin to top.
menu-more = 더보기
menu-move-to-heading = 이동할 위치:
menu-find-from = { $name }님이 보낸 메일 찾기

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] 대화 { $count }개가 보관처리되었습니다.
   *[message] 메일 { $count }개가 보관처리되었습니다.
}
toast-trashed = { $kind ->
    [conversation] 대화 { $count }개가 휴지통으로 이동되었습니다.
   *[message] 메일 { $count }개가 휴지통으로 이동되었습니다.
}
toast-moved = { $kind ->
    [conversation] 대화 { $count }개가 이동되었습니다.
   *[message] 메일 { $count }개가 이동되었습니다.
}
toast-starred = { $kind ->
    [conversation] 대화 { $count }개에 별표를 추가했습니다.
   *[message] 메일 { $count }개에 별표를 추가했습니다.
}
toast-unstarred = { $kind ->
    [conversation] 대화 { $count }개의 별표를 삭제했습니다.
   *[message] 메일 { $count }개의 별표를 삭제했습니다.
}
toast-important = { $kind ->
    [conversation] 대화 { $count }개를 중요로 표시했습니다.
   *[message] 메일 { $count }개를 중요로 표시했습니다.
}
toast-not-important = { $kind ->
    [conversation] 대화 { $count }개를 중요하지 않음으로 표시했습니다.
   *[message] 메일 { $count }개를 중요하지 않음으로 표시했습니다.
}
toast-pinned = { $kind ->
    [conversation] 대화 { $count }개를 상단에 고정했습니다.
   *[message] 메일 { $count }개를 상단에 고정했습니다.
}
toast-unpinned = { $kind ->
    [conversation] 대화 { $count }개의 고정을 해제했습니다.
   *[message] 메일 { $count }개의 고정을 해제했습니다.
}
toast-snoozed = { $kind ->
    [conversation] 대화 { $count }개를 { $when }에 다시 알림으로 설정했습니다.
   *[message] 메일 { $count }개를 { $when }에 다시 알림으로 설정했습니다.
}
toast-unsnoozed = { $kind ->
    [conversation] 대화 { $count }개가 받은편지함으로 돌아왔습니다.
   *[message] 메일 { $count }개가 받은편지함으로 돌아왔습니다.
}
toast-spam = { $kind ->
    [conversation] 대화 { $count }개를 스팸으로 신고했습니다.
   *[message] 메일 { $count }개를 스팸으로 신고했습니다.
}
toast-not-spam = { $kind ->
    [conversation] 대화 { $count }개를 스팸 아님으로 표시하고 받은편지함으로 이동했습니다.
   *[message] 메일 { $count }개를 스팸 아님으로 표시하고 받은편지함으로 이동했습니다.
}
toast-deleted-forever = { $kind ->
    [conversation] 대화 { $count }개를 영구삭제했습니다.
   *[message] 메일 { $count }개를 영구삭제했습니다.
}
toast-marked-read = { $kind ->
    [conversation] 대화 { $count }개를 읽음으로 표시했습니다.
   *[message] 메일 { $count }개를 읽음으로 표시했습니다.
}
toast-marked-unread = { $kind ->
    [conversation] 대화 { $count }개를 읽지 않음으로 표시했습니다.
   *[message] 메일 { $count }개를 읽지 않음으로 표시했습니다.
}
toast-undone = 작업을 실행취소했습니다.
toast-nothing-to-undo = 실행취소할 작업이 없습니다.
toast-cannot-undo-delete-forever = 영구삭제한 메일은 되돌릴 수 없습니다.
toast-send-undone = 보내기를 취소했습니다.
toast-too-late-to-undo-send = 실행취소하기에는 너무 늦었습니다: 메일이 이미 전송되었습니다.
toast-undo = 실행취소
toast-close = 닫기
toast-no-spam-folder = 이 계정에는 스팸함이 없습니다.
