# Katna Mail, Spanish (Spain) (Español).
# Machine-drafted by AI; not yet reviewed by a native speaker.
# Corrections welcome: see i18n/README.md.

## Mail list: inbox tabs (Gmail's categories, Outlook's and Zoho's)

tab-primary = Principal
tab-promotions = Promociones
tab-social = Social
tab-updates = Notificaciones
tab-forums = Foros
tab-focused = Prioritarios
tab-other = Otros
tab-inbox = Bandeja de entrada
tab-newsletters = Boletines
tab-notifications = Notificaciones
tab-new = { $count ->
    [one] { $count } nuevo
    [many] { $count } de nuevos
   *[other] { $count } nuevos
}
tab-provider-other = ordenado por Katna

## Mail list: toolbar

list-select = Seleccionar
list-refresh = Actualizar
list-more = Más
list-mark-read = Marcar como leído
list-mark-unread = Marcar como no leído
list-move-to = Mover a
list-archive = Archivar
list-spam = Marcar como spam
list-delete = Eliminar
list-newer = Más recientes
list-older = Más antiguos
list-range = { $first }–{ $last } de { $total }
list-range-about = { $first }–{ $last } de unos { $total }
list-results = Resultados de «{ $query }»
list-results-corrected = Mostrando resultados de «{ $query }»
list-search-instead = Buscar «{ $query }» en su lugar
list-files-more = +{ $count }

## Mail list: Select menu (which lines to tick)

list-pick-all = Todos
list-pick-none = Ninguno
list-pick-read = Leídos
list-pick-unread = No leídos
list-pick-starred = Destacados
list-pick-unstarred = No destacados

## Mail list: banner when every line is ticked

list-selected-all = { $kind ->
    [conversation] { $count ->
        [one] Se ha seleccionado { $count } conversación.
        [many] Se han seleccionado las { $count } de conversaciones.
       *[other] Se han seleccionado las { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha seleccionado { $count } mensaje.
        [many] Se han seleccionado los { $count } de mensajes.
       *[other] Se han seleccionado los { $count } mensajes.
    }
}
list-selected-all-in = { $kind ->
    [conversation] { $count ->
        [one] Se ha seleccionado { $count } conversación de { $folder }.
        [many] Se han seleccionado las { $count } de conversaciones de { $folder }.
       *[other] Se han seleccionado las { $count } conversaciones de { $folder }.
    }
   *[message] { $count ->
        [one] Se ha seleccionado { $count } mensaje de { $folder }.
        [many] Se han seleccionado los { $count } de mensajes de { $folder }.
       *[other] Se han seleccionado los { $count } mensajes de { $folder }.
    }
}
list-selected-screen = { $kind ->
    [conversation] { $count ->
        [one] Se ha seleccionado { $count } conversación de esta página.
        [many] Se han seleccionado las { $count } de conversaciones de esta página.
       *[other] Se han seleccionado las { $count } conversaciones de esta página.
    }
   *[message] { $count ->
        [one] Se ha seleccionado { $count } mensaje de esta página.
        [many] Se han seleccionado los { $count } de mensajes de esta página.
       *[other] Se han seleccionado los { $count } mensajes de esta página.
    }
}
list-select-all = { $kind ->
    [conversation] { $count ->
        [one] Seleccionar { $count } conversación
        [many] Seleccionar las { $count } de conversaciones
       *[other] Seleccionar las { $count } conversaciones
    }
   *[message] { $count ->
        [one] Seleccionar { $count } mensaje
        [many] Seleccionar los { $count } de mensajes
       *[other] Seleccionar los { $count } mensajes
    }
}
list-select-all-in = { $kind ->
    [conversation] { $count ->
        [one] Seleccionar { $count } conversación de { $folder }
        [many] Seleccionar las { $count } de conversaciones de { $folder }
       *[other] Seleccionar las { $count } conversaciones de { $folder }
    }
   *[message] { $count ->
        [one] Seleccionar { $count } mensaje de { $folder }
        [many] Seleccionar los { $count } de mensajes de { $folder }
       *[other] Seleccionar los { $count } mensajes de { $folder }
    }
}
list-selected-picked-screen = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación leída de esta página.
            [many] Se han seleccionado las { $count } de conversaciones leídas de esta página.
           *[other] Se han seleccionado las { $count } conversaciones leídas de esta página.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje leído de esta página.
            [many] Se han seleccionado los { $count } de mensajes leídos de esta página.
           *[other] Se han seleccionado los { $count } mensajes leídos de esta página.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación no leída de esta página.
            [many] Se han seleccionado las { $count } de conversaciones no leídas de esta página.
           *[other] Se han seleccionado las { $count } conversaciones no leídas de esta página.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje no leído de esta página.
            [many] Se han seleccionado los { $count } de mensajes no leídos de esta página.
           *[other] Se han seleccionado los { $count } mensajes no leídos de esta página.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación destacada de esta página.
            [many] Se han seleccionado las { $count } de conversaciones destacadas de esta página.
           *[other] Se han seleccionado las { $count } conversaciones destacadas de esta página.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje destacado de esta página.
            [many] Se han seleccionado los { $count } de mensajes destacados de esta página.
           *[other] Se han seleccionado los { $count } mensajes destacados de esta página.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación no destacada de esta página.
            [many] Se han seleccionado las { $count } de conversaciones no destacadas de esta página.
           *[other] Se han seleccionado las { $count } conversaciones no destacadas de esta página.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje no destacado de esta página.
            [many] Se han seleccionado los { $count } de mensajes no destacados de esta página.
           *[other] Se han seleccionado los { $count } mensajes no destacados de esta página.
        }
    }
}
list-select-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Seleccionar { $count } conversación leída
            [many] Seleccionar las { $count } de conversaciones leídas
           *[other] Seleccionar las { $count } conversaciones leídas
        }
       *[message] { $count ->
            [one] Seleccionar { $count } mensaje leído
            [many] Seleccionar los { $count } de mensajes leídos
           *[other] Seleccionar los { $count } mensajes leídos
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Seleccionar { $count } conversación no leída
            [many] Seleccionar las { $count } de conversaciones no leídas
           *[other] Seleccionar las { $count } conversaciones no leídas
        }
       *[message] { $count ->
            [one] Seleccionar { $count } mensaje no leído
            [many] Seleccionar los { $count } de mensajes no leídos
           *[other] Seleccionar los { $count } mensajes no leídos
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Seleccionar { $count } conversación destacada
            [many] Seleccionar las { $count } de conversaciones destacadas
           *[other] Seleccionar las { $count } conversaciones destacadas
        }
       *[message] { $count ->
            [one] Seleccionar { $count } mensaje destacado
            [many] Seleccionar los { $count } de mensajes destacados
           *[other] Seleccionar los { $count } mensajes destacados
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Seleccionar { $count } conversación no destacada
            [many] Seleccionar las { $count } de conversaciones no destacadas
           *[other] Seleccionar las { $count } conversaciones no destacadas
        }
       *[message] { $count ->
            [one] Seleccionar { $count } mensaje no destacado
            [many] Seleccionar los { $count } de mensajes no destacados
           *[other] Seleccionar los { $count } mensajes no destacados
        }
    }
}
list-select-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Seleccionar { $count } conversación leída de { $folder }
            [many] Seleccionar las { $count } de conversaciones leídas de { $folder }
           *[other] Seleccionar las { $count } conversaciones leídas de { $folder }
        }
       *[message] { $count ->
            [one] Seleccionar { $count } mensaje leído de { $folder }
            [many] Seleccionar los { $count } de mensajes leídos de { $folder }
           *[other] Seleccionar los { $count } mensajes leídos de { $folder }
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Seleccionar { $count } conversación no leída de { $folder }
            [many] Seleccionar las { $count } de conversaciones no leídas de { $folder }
           *[other] Seleccionar las { $count } conversaciones no leídas de { $folder }
        }
       *[message] { $count ->
            [one] Seleccionar { $count } mensaje no leído de { $folder }
            [many] Seleccionar los { $count } de mensajes no leídos de { $folder }
           *[other] Seleccionar los { $count } mensajes no leídos de { $folder }
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Seleccionar { $count } conversación destacada de { $folder }
            [many] Seleccionar las { $count } de conversaciones destacadas de { $folder }
           *[other] Seleccionar las { $count } conversaciones destacadas de { $folder }
        }
       *[message] { $count ->
            [one] Seleccionar { $count } mensaje destacado de { $folder }
            [many] Seleccionar los { $count } de mensajes destacados de { $folder }
           *[other] Seleccionar los { $count } mensajes destacados de { $folder }
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Seleccionar { $count } conversación no destacada de { $folder }
            [many] Seleccionar las { $count } de conversaciones no destacadas de { $folder }
           *[other] Seleccionar las { $count } conversaciones no destacadas de { $folder }
        }
       *[message] { $count ->
            [one] Seleccionar { $count } mensaje no destacado de { $folder }
            [many] Seleccionar los { $count } de mensajes no destacados de { $folder }
           *[other] Seleccionar los { $count } mensajes no destacados de { $folder }
        }
    }
}
list-selected-picked = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación leída.
            [many] Se han seleccionado las { $count } de conversaciones leídas.
           *[other] Se han seleccionado las { $count } conversaciones leídas.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje leído.
            [many] Se han seleccionado los { $count } de mensajes leídos.
           *[other] Se han seleccionado los { $count } mensajes leídos.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación no leída.
            [many] Se han seleccionado las { $count } de conversaciones no leídas.
           *[other] Se han seleccionado las { $count } conversaciones no leídas.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje no leído.
            [many] Se han seleccionado los { $count } de mensajes no leídos.
           *[other] Se han seleccionado los { $count } mensajes no leídos.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación destacada.
            [many] Se han seleccionado las { $count } de conversaciones destacadas.
           *[other] Se han seleccionado las { $count } conversaciones destacadas.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje destacado.
            [many] Se han seleccionado los { $count } de mensajes destacados.
           *[other] Se han seleccionado los { $count } mensajes destacados.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación no destacada.
            [many] Se han seleccionado las { $count } de conversaciones no destacadas.
           *[other] Se han seleccionado las { $count } conversaciones no destacadas.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje no destacado.
            [many] Se han seleccionado los { $count } de mensajes no destacados.
           *[other] Se han seleccionado los { $count } mensajes no destacados.
        }
    }
}
list-selected-picked-in = { $pick ->
    [read] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación leída en { $folder }.
            [many] Se han seleccionado las { $count } de conversaciones leídas en { $folder }.
           *[other] Se han seleccionado las { $count } conversaciones leídas en { $folder }.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje leído en { $folder }.
            [many] Se han seleccionado los { $count } de mensajes leídos en { $folder }.
           *[other] Se han seleccionado los { $count } mensajes leídos en { $folder }.
        }
    }
   *[unread] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación no leída en { $folder }.
            [many] Se han seleccionado las { $count } de conversaciones no leídas en { $folder }.
           *[other] Se han seleccionado las { $count } conversaciones no leídas en { $folder }.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje no leído en { $folder }.
            [many] Se han seleccionado los { $count } de mensajes no leídos en { $folder }.
           *[other] Se han seleccionado los { $count } mensajes no leídos en { $folder }.
        }
    }
    [starred] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación destacada en { $folder }.
            [many] Se han seleccionado las { $count } de conversaciones destacadas en { $folder }.
           *[other] Se han seleccionado las { $count } conversaciones destacadas en { $folder }.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje destacado en { $folder }.
            [many] Se han seleccionado los { $count } de mensajes destacados en { $folder }.
           *[other] Se han seleccionado los { $count } mensajes destacados en { $folder }.
        }
    }
    [unstarred] { $kind ->
        [conversation] { $count ->
            [one] Se ha seleccionado { $count } conversación no destacada en { $folder }.
            [many] Se han seleccionado las { $count } de conversaciones no destacadas en { $folder }.
           *[other] Se han seleccionado las { $count } conversaciones no destacadas en { $folder }.
        }
       *[message] { $count ->
            [one] Se ha seleccionado { $count } mensaje no destacado en { $folder }.
            [many] Se han seleccionado los { $count } de mensajes no destacados en { $folder }.
           *[other] Se han seleccionado los { $count } mensajes no destacados en { $folder }.
        }
    }
}
list-picked-none = { $pick ->
    [read] { $kind ->
        [conversation] No hay conversaciones leídas aquí.
       *[message] No hay mensajes leídos aquí.
    }
   *[unread] { $kind ->
        [conversation] No hay conversaciones no leídas aquí.
       *[message] No hay mensajes no leídos aquí.
    }
    [starred] { $kind ->
        [conversation] No hay conversaciones destacadas aquí.
       *[message] No hay mensajes destacados aquí.
    }
    [unstarred] { $kind ->
        [conversation] No hay conversaciones no destacadas aquí.
       *[message] No hay mensajes no destacados aquí.
    }
}
list-clear-selection = Borrar selección

## Mail list: empty states

list-empty-search = Ningún mensaje coincide con tu búsqueda.
list-empty-tab = No hay correo en { $tab }.
list-empty-tab-unknown = No hay correo en esta pestaña.
list-empty-folder = No hay mensajes en { $folder }.
list-empty-folder-unknown = No hay mensajes en esta carpeta.
list-first-sync = Obteniendo tu correo…
list-first-sync-detail = Aparecerá aquí a medida que llegue.

## Mail list: lines

row-removed = Este mensaje se ha eliminado.
row-starred = Destacado
row-not-starred = No destacado
row-important = Importante. Haz clic para marcarlo como no importante.
row-mark-important = Marcar como importante
row-pinned = Fijado arriba
row-pin = Fijar arriba
row-unpin = No fijar

## Mail list: More menu and right-click menu

menu-reply = Responder
menu-reply-all = Responder a todos
menu-forward = Reenviar
menu-archive = Archivar
menu-delete = Eliminar
menu-delete-forever = Eliminar definitivamente
menu-move-to-inbox = Mover a Recibidos
menu-spam = Marcar como spam
menu-not-spam = No es spam
menu-mark-read = Marcar como leído
menu-mark-unread = Marcar como no leído
menu-mark-all-read = Marcar todo como leído
menu-star = Añadir estrella
menu-unstar = Quitar estrella
menu-important = Marcar como importante
menu-not-important = Marcar como no importante
menu-pin = Fijar arriba
menu-unpin = No fijar
menu-print-all = Imprimir todo
menu-new-window = Abrir en una ventana nueva
menu-move-to = Mover a
menu-move-to-heading = Mover a:
menu-find-from = Buscar correos de { $name }

## Snackbar after an action on mail in the list

toast-archived = { $kind ->
    [conversation] { $count ->
        [one] Se ha archivado la conversación.
        [many] Se han archivado { $count } de conversaciones.
       *[other] Se han archivado { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha archivado el mensaje.
        [many] Se han archivado { $count } de mensajes.
       *[other] Se han archivado { $count } mensajes.
    }
}
toast-trashed = { $kind ->
    [conversation] { $count ->
        [one] Se ha movido la conversación a la papelera.
        [many] Se han movido { $count } de conversaciones a la papelera.
       *[other] Se han movido { $count } conversaciones a la papelera.
    }
   *[message] { $count ->
        [one] Se ha movido el mensaje a la papelera.
        [many] Se han movido { $count } de mensajes a la papelera.
       *[other] Se han movido { $count } mensajes a la papelera.
    }
}
toast-moved = { $kind ->
    [conversation] { $count ->
        [one] Se ha movido la conversación.
        [many] Se han movido { $count } de conversaciones.
       *[other] Se han movido { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha movido el mensaje.
        [many] Se han movido { $count } de mensajes.
       *[other] Se han movido { $count } mensajes.
    }
}
toast-starred = { $kind ->
    [conversation] { $count ->
        [one] Se ha destacado la conversación.
        [many] Se han destacado { $count } de conversaciones.
       *[other] Se han destacado { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha destacado el mensaje.
        [many] Se han destacado { $count } de mensajes.
       *[other] Se han destacado { $count } mensajes.
    }
}
toast-unstarred = { $kind ->
    [conversation] { $count ->
        [one] Se ha quitado la estrella de la conversación.
        [many] Se ha quitado la estrella de { $count } de conversaciones.
       *[other] Se ha quitado la estrella de { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha quitado la estrella del mensaje.
        [many] Se ha quitado la estrella de { $count } de mensajes.
       *[other] Se ha quitado la estrella de { $count } mensajes.
    }
}
toast-important = { $kind ->
    [conversation] { $count ->
        [one] Se ha marcado la conversación como importante.
        [many] Se han marcado { $count } de conversaciones como importantes.
       *[other] Se han marcado { $count } conversaciones como importantes.
    }
   *[message] { $count ->
        [one] Se ha marcado el mensaje como importante.
        [many] Se han marcado { $count } de mensajes como importantes.
       *[other] Se han marcado { $count } mensajes como importantes.
    }
}
toast-not-important = { $kind ->
    [conversation] { $count ->
        [one] Se ha marcado la conversación como no importante.
        [many] Se han marcado { $count } de conversaciones como no importantes.
       *[other] Se han marcado { $count } conversaciones como no importantes.
    }
   *[message] { $count ->
        [one] Se ha marcado el mensaje como no importante.
        [many] Se han marcado { $count } de mensajes como no importantes.
       *[other] Se han marcado { $count } mensajes como no importantes.
    }
}
toast-pinned = { $kind ->
    [conversation] { $count ->
        [one] Se ha fijado la conversación arriba.
        [many] Se han fijado { $count } de conversaciones arriba.
       *[other] Se han fijado { $count } conversaciones arriba.
    }
   *[message] { $count ->
        [one] Se ha fijado el mensaje arriba.
        [many] Se han fijado { $count } de mensajes arriba.
       *[other] Se han fijado { $count } mensajes arriba.
    }
}
toast-unpinned = { $kind ->
    [conversation] { $count ->
        [one] Se ha dejado de fijar la conversación.
        [many] Se han dejado de fijar { $count } de conversaciones.
       *[other] Se han dejado de fijar { $count } conversaciones.
    }
   *[message] { $count ->
        [one] Se ha dejado de fijar el mensaje.
        [many] Se han dejado de fijar { $count } de mensajes.
       *[other] Se han dejado de fijar { $count } mensajes.
    }
}
toast-spam = { $kind ->
    [conversation] { $count ->
        [one] Se ha marcado la conversación como spam.
        [many] Se han marcado { $count } de conversaciones como spam.
       *[other] Se han marcado { $count } conversaciones como spam.
    }
   *[message] { $count ->
        [one] Se ha marcado el mensaje como spam.
        [many] Se han marcado { $count } de mensajes como spam.
       *[other] Se han marcado { $count } mensajes como spam.
    }
}
toast-not-spam = { $kind ->
    [conversation] { $count ->
        [one] Se ha marcado la conversación como no spam y se ha movido a Recibidos.
        [many] Se han marcado { $count } de conversaciones como no spam y se han movido a Recibidos.
       *[other] Se han marcado { $count } conversaciones como no spam y se han movido a Recibidos.
    }
   *[message] { $count ->
        [one] Se ha marcado el mensaje como no spam y se ha movido a Recibidos.
        [many] Se han marcado { $count } de mensajes como no spam y se han movido a Recibidos.
       *[other] Se han marcado { $count } mensajes como no spam y se han movido a Recibidos.
    }
}
toast-deleted-forever = { $kind ->
    [conversation] { $count ->
        [one] Se ha eliminado la conversación definitivamente.
        [many] Se han eliminado { $count } de conversaciones definitivamente.
       *[other] Se han eliminado { $count } conversaciones definitivamente.
    }
   *[message] { $count ->
        [one] Se ha eliminado el mensaje definitivamente.
        [many] Se han eliminado { $count } de mensajes definitivamente.
       *[other] Se han eliminado { $count } mensajes definitivamente.
    }
}
toast-marked-read = { $kind ->
    [conversation] { $count ->
        [one] Se ha marcado la conversación como leída.
        [many] Se han marcado { $count } de conversaciones como leídas.
       *[other] Se han marcado { $count } conversaciones como leídas.
    }
   *[message] { $count ->
        [one] Se ha marcado el mensaje como leído.
        [many] Se han marcado { $count } de mensajes como leídos.
       *[other] Se han marcado { $count } mensajes como leídos.
    }
}
toast-marked-unread = { $kind ->
    [conversation] { $count ->
        [one] Se ha marcado la conversación como no leída.
        [many] Se han marcado { $count } de conversaciones como no leídas.
       *[other] Se han marcado { $count } conversaciones como no leídas.
    }
   *[message] { $count ->
        [one] Se ha marcado el mensaje como no leído.
        [many] Se han marcado { $count } de mensajes como no leídos.
       *[other] Se han marcado { $count } mensajes como no leídos.
    }
}
toast-undone = Se ha deshecho la acción.
toast-nothing-to-undo = No hay nada que deshacer.
toast-cannot-undo-delete-forever = El correo eliminado definitivamente no se puede recuperar.
toast-send-undone = Se ha deshecho el envío.
toast-too-late-to-undo-send = Demasiado tarde para deshacer: el mensaje ya se ha enviado.
toast-undo = Deshacer
toast-no-spam-folder = Esta cuenta no tiene carpeta de spam.
