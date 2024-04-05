Files containing some information about APIs available on Synology
DSM.

Obtained from
* DSM version: 7.2.1-69057 Update 3
* Location: `/usr/syno/synoman/webapi/`

The same information can be obtained from the API itself, for example
using `curl` to query all available APIs:

```
curl -q "https://{dsm_url}:5001/webapi/entry.cgi?api=SYNO.API.Info&version=1&method=query"
```

Using `query` param you can filter the list down to specific API,
e.g.:

```
curl -q "https://{dsm_url}:5001/webapi/entry.cgi?api=SYNO.API.Info&version=1&method=query&query=SYNO.Foto.BackgroundTask.File"
```
