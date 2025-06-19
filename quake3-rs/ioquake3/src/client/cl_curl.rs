use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::wchar_t;
pub use crate::stdlib::__uint8_t;

pub use crate::stdlib::C2RustUnnamed_18;
pub use crate::stdlib::__fd_mask;
pub use crate::stdlib::__mbstate_t;
pub use crate::stdlib::fd_set;
pub use crate::stdlib::uint8_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::curlbuild_h::curl_off_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::src::qcommon::common::com_developer;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_SetValue;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_SV_FOpenFileWrite;
pub use crate::src::qcommon::files::FS_SV_Rename;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::net_ip::NET_AdrToString;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

pub use crate::curl_h::CURLcode;
pub use crate::curl_h::CURLoption;
pub use crate::curl_h::CURL;
pub use crate::curl_h::CURLE_ABORTED_BY_CALLBACK;
pub use crate::curl_h::CURLE_AGAIN;
pub use crate::curl_h::CURLE_BAD_CONTENT_ENCODING;
pub use crate::curl_h::CURLE_BAD_DOWNLOAD_RESUME;
pub use crate::curl_h::CURLE_BAD_FUNCTION_ARGUMENT;
pub use crate::curl_h::CURLE_CHUNK_FAILED;
pub use crate::curl_h::CURLE_CONV_FAILED;
pub use crate::curl_h::CURLE_CONV_REQD;
pub use crate::curl_h::CURLE_COULDNT_CONNECT;
pub use crate::curl_h::CURLE_COULDNT_RESOLVE_HOST;
pub use crate::curl_h::CURLE_COULDNT_RESOLVE_PROXY;
pub use crate::curl_h::CURLE_FAILED_INIT;
pub use crate::curl_h::CURLE_FILESIZE_EXCEEDED;
pub use crate::curl_h::CURLE_FILE_COULDNT_READ_FILE;
pub use crate::curl_h::CURLE_FTP_ACCEPT_FAILED;
pub use crate::curl_h::CURLE_FTP_ACCEPT_TIMEOUT;
pub use crate::curl_h::CURLE_FTP_BAD_FILE_LIST;
pub use crate::curl_h::CURLE_FTP_CANT_GET_HOST;
pub use crate::curl_h::CURLE_FTP_COULDNT_RETR_FILE;
pub use crate::curl_h::CURLE_FTP_COULDNT_SET_TYPE;
pub use crate::curl_h::CURLE_FTP_COULDNT_USE_REST;
pub use crate::curl_h::CURLE_FTP_PORT_FAILED;
pub use crate::curl_h::CURLE_FTP_PRET_FAILED;
pub use crate::curl_h::CURLE_FTP_WEIRD_227_FORMAT;
pub use crate::curl_h::CURLE_FTP_WEIRD_PASS_REPLY;
pub use crate::curl_h::CURLE_FTP_WEIRD_PASV_REPLY;
pub use crate::curl_h::CURLE_FUNCTION_NOT_FOUND;
pub use crate::curl_h::CURLE_GOT_NOTHING;
pub use crate::curl_h::CURLE_HTTP2;
pub use crate::curl_h::CURLE_HTTP2_STREAM;
pub use crate::curl_h::CURLE_HTTP_POST_ERROR;
pub use crate::curl_h::CURLE_HTTP_RETURNED_ERROR;
pub use crate::curl_h::CURLE_INTERFACE_FAILED;
pub use crate::curl_h::CURLE_LDAP_CANNOT_BIND;
pub use crate::curl_h::CURLE_LDAP_INVALID_URL;
pub use crate::curl_h::CURLE_LDAP_SEARCH_FAILED;
pub use crate::curl_h::CURLE_LOGIN_DENIED;
pub use crate::curl_h::CURLE_NOT_BUILT_IN;
pub use crate::curl_h::CURLE_NO_CONNECTION_AVAILABLE;
pub use crate::curl_h::CURLE_OBSOLETE20;
pub use crate::curl_h::CURLE_OBSOLETE24;
pub use crate::curl_h::CURLE_OBSOLETE29;
pub use crate::curl_h::CURLE_OBSOLETE32;
pub use crate::curl_h::CURLE_OBSOLETE40;
pub use crate::curl_h::CURLE_OBSOLETE44;
pub use crate::curl_h::CURLE_OBSOLETE46;
pub use crate::curl_h::CURLE_OBSOLETE50;
pub use crate::curl_h::CURLE_OBSOLETE57;
pub use crate::curl_h::CURLE_OK;
pub use crate::curl_h::CURLE_OPERATION_TIMEDOUT;
pub use crate::curl_h::CURLE_OUT_OF_MEMORY;
pub use crate::curl_h::CURLE_PARTIAL_FILE;
pub use crate::curl_h::CURLE_PEER_FAILED_VERIFICATION;
pub use crate::curl_h::CURLE_QUOTE_ERROR;
pub use crate::curl_h::CURLE_RANGE_ERROR;
pub use crate::curl_h::CURLE_READ_ERROR;
pub use crate::curl_h::CURLE_RECV_ERROR;
pub use crate::curl_h::CURLE_REMOTE_ACCESS_DENIED;
pub use crate::curl_h::CURLE_REMOTE_DISK_FULL;
pub use crate::curl_h::CURLE_REMOTE_FILE_EXISTS;
pub use crate::curl_h::CURLE_REMOTE_FILE_NOT_FOUND;
pub use crate::curl_h::CURLE_RTSP_CSEQ_ERROR;
pub use crate::curl_h::CURLE_RTSP_SESSION_ERROR;
pub use crate::curl_h::CURLE_SEND_ERROR;
pub use crate::curl_h::CURLE_SEND_FAIL_REWIND;
pub use crate::curl_h::CURLE_SSH;
pub use crate::curl_h::CURLE_SSL_CACERT;
pub use crate::curl_h::CURLE_SSL_CACERT_BADFILE;
pub use crate::curl_h::CURLE_SSL_CERTPROBLEM;
pub use crate::curl_h::CURLE_SSL_CIPHER;
pub use crate::curl_h::CURLE_SSL_CONNECT_ERROR;
pub use crate::curl_h::CURLE_SSL_CRL_BADFILE;
pub use crate::curl_h::CURLE_SSL_ENGINE_INITFAILED;
pub use crate::curl_h::CURLE_SSL_ENGINE_NOTFOUND;
pub use crate::curl_h::CURLE_SSL_ENGINE_SETFAILED;
pub use crate::curl_h::CURLE_SSL_INVALIDCERTSTATUS;
pub use crate::curl_h::CURLE_SSL_ISSUER_ERROR;
pub use crate::curl_h::CURLE_SSL_PINNEDPUBKEYNOTMATCH;
pub use crate::curl_h::CURLE_SSL_SHUTDOWN_FAILED;
pub use crate::curl_h::CURLE_TELNET_OPTION_SYNTAX;
pub use crate::curl_h::CURLE_TFTP_ILLEGAL;
pub use crate::curl_h::CURLE_TFTP_NOSUCHUSER;
pub use crate::curl_h::CURLE_TFTP_NOTFOUND;
pub use crate::curl_h::CURLE_TFTP_PERM;
pub use crate::curl_h::CURLE_TFTP_UNKNOWNID;
pub use crate::curl_h::CURLE_TOO_MANY_REDIRECTS;
pub use crate::curl_h::CURLE_UNKNOWN_OPTION;
pub use crate::curl_h::CURLE_UNSUPPORTED_PROTOCOL;
pub use crate::curl_h::CURLE_UPLOAD_FAILED;
pub use crate::curl_h::CURLE_URL_MALFORMAT;
pub use crate::curl_h::CURLE_USE_SSL_FAILED;
pub use crate::curl_h::CURLE_WEIRD_SERVER_REPLY;
pub use crate::curl_h::CURLE_WRITE_ERROR;
pub use crate::curl_h::CURLINFO;
pub use crate::curl_h::CURLINFO_ACTIVESOCKET;
pub use crate::curl_h::CURLINFO_APPCONNECT_TIME;
pub use crate::curl_h::CURLINFO_CERTINFO;
pub use crate::curl_h::CURLINFO_CONDITION_UNMET;
pub use crate::curl_h::CURLINFO_CONNECT_TIME;
pub use crate::curl_h::CURLINFO_CONTENT_LENGTH_DOWNLOAD;
pub use crate::curl_h::CURLINFO_CONTENT_LENGTH_UPLOAD;
pub use crate::curl_h::CURLINFO_CONTENT_TYPE;
pub use crate::curl_h::CURLINFO_COOKIELIST;
pub use crate::curl_h::CURLINFO_EFFECTIVE_URL;
pub use crate::curl_h::CURLINFO_FILETIME;
pub use crate::curl_h::CURLINFO_FTP_ENTRY_PATH;
pub use crate::curl_h::CURLINFO_HEADER_SIZE;
pub use crate::curl_h::CURLINFO_HTTPAUTH_AVAIL;
pub use crate::curl_h::CURLINFO_HTTP_CONNECTCODE;
pub use crate::curl_h::CURLINFO_HTTP_VERSION;
pub use crate::curl_h::CURLINFO_LASTONE;
pub use crate::curl_h::CURLINFO_LASTSOCKET;
pub use crate::curl_h::CURLINFO_LOCAL_IP;
pub use crate::curl_h::CURLINFO_LOCAL_PORT;
pub use crate::curl_h::CURLINFO_NAMELOOKUP_TIME;
pub use crate::curl_h::CURLINFO_NONE;
pub use crate::curl_h::CURLINFO_NUM_CONNECTS;
pub use crate::curl_h::CURLINFO_OS_ERRNO;
pub use crate::curl_h::CURLINFO_PRETRANSFER_TIME;
pub use crate::curl_h::CURLINFO_PRIMARY_IP;
pub use crate::curl_h::CURLINFO_PRIMARY_PORT;
pub use crate::curl_h::CURLINFO_PRIVATE;
pub use crate::curl_h::CURLINFO_PROTOCOL;
pub use crate::curl_h::CURLINFO_PROXYAUTH_AVAIL;
pub use crate::curl_h::CURLINFO_PROXY_SSL_VERIFYRESULT;
pub use crate::curl_h::CURLINFO_REDIRECT_COUNT;
pub use crate::curl_h::CURLINFO_REDIRECT_TIME;
pub use crate::curl_h::CURLINFO_REDIRECT_URL;
pub use crate::curl_h::CURLINFO_REQUEST_SIZE;
pub use crate::curl_h::CURLINFO_RESPONSE_CODE;
pub use crate::curl_h::CURLINFO_RTSP_CLIENT_CSEQ;
pub use crate::curl_h::CURLINFO_RTSP_CSEQ_RECV;
pub use crate::curl_h::CURLINFO_RTSP_SERVER_CSEQ;
pub use crate::curl_h::CURLINFO_RTSP_SESSION_ID;
pub use crate::curl_h::CURLINFO_SCHEME;
pub use crate::curl_h::CURLINFO_SIZE_DOWNLOAD;
pub use crate::curl_h::CURLINFO_SIZE_UPLOAD;
pub use crate::curl_h::CURLINFO_SPEED_DOWNLOAD;
pub use crate::curl_h::CURLINFO_SPEED_UPLOAD;
pub use crate::curl_h::CURLINFO_SSL_ENGINES;
pub use crate::curl_h::CURLINFO_SSL_VERIFYRESULT;
pub use crate::curl_h::CURLINFO_STARTTRANSFER_TIME;
pub use crate::curl_h::CURLINFO_TLS_SESSION;
pub use crate::curl_h::CURLINFO_TLS_SSL_PTR;
pub use crate::curl_h::CURLINFO_TOTAL_TIME;
pub use crate::curl_h::CURLOPT_ABSTRACT_UNIX_SOCKET;
pub use crate::curl_h::CURLOPT_ACCEPTTIMEOUT_MS;
pub use crate::curl_h::CURLOPT_ACCEPT_ENCODING;
pub use crate::curl_h::CURLOPT_ADDRESS_SCOPE;
pub use crate::curl_h::CURLOPT_APPEND;
pub use crate::curl_h::CURLOPT_AUTOREFERER;
pub use crate::curl_h::CURLOPT_BUFFERSIZE;
pub use crate::curl_h::CURLOPT_CAINFO;
pub use crate::curl_h::CURLOPT_CAPATH;
pub use crate::curl_h::CURLOPT_CERTINFO;
pub use crate::curl_h::CURLOPT_CHUNK_BGN_FUNCTION;
pub use crate::curl_h::CURLOPT_CHUNK_DATA;
pub use crate::curl_h::CURLOPT_CHUNK_END_FUNCTION;
pub use crate::curl_h::CURLOPT_CLOSESOCKETDATA;
pub use crate::curl_h::CURLOPT_CLOSESOCKETFUNCTION;
pub use crate::curl_h::CURLOPT_CONNECTTIMEOUT;
pub use crate::curl_h::CURLOPT_CONNECTTIMEOUT_MS;
pub use crate::curl_h::CURLOPT_CONNECT_ONLY;
pub use crate::curl_h::CURLOPT_CONNECT_TO;
pub use crate::curl_h::CURLOPT_CONV_FROM_NETWORK_FUNCTION;
pub use crate::curl_h::CURLOPT_CONV_FROM_UTF8_FUNCTION;
pub use crate::curl_h::CURLOPT_CONV_TO_NETWORK_FUNCTION;
pub use crate::curl_h::CURLOPT_COOKIE;
pub use crate::curl_h::CURLOPT_COOKIEFILE;
pub use crate::curl_h::CURLOPT_COOKIEJAR;
pub use crate::curl_h::CURLOPT_COOKIELIST;
pub use crate::curl_h::CURLOPT_COOKIESESSION;
pub use crate::curl_h::CURLOPT_COPYPOSTFIELDS;
pub use crate::curl_h::CURLOPT_CRLF;
pub use crate::curl_h::CURLOPT_CRLFILE;
pub use crate::curl_h::CURLOPT_CUSTOMREQUEST;
pub use crate::curl_h::CURLOPT_DEBUGDATA;
pub use crate::curl_h::CURLOPT_DEBUGFUNCTION;
pub use crate::curl_h::CURLOPT_DEFAULT_PROTOCOL;
pub use crate::curl_h::CURLOPT_DIRLISTONLY;
pub use crate::curl_h::CURLOPT_DNS_CACHE_TIMEOUT;
pub use crate::curl_h::CURLOPT_DNS_INTERFACE;
pub use crate::curl_h::CURLOPT_DNS_LOCAL_IP4;
pub use crate::curl_h::CURLOPT_DNS_LOCAL_IP6;
pub use crate::curl_h::CURLOPT_DNS_SERVERS;
pub use crate::curl_h::CURLOPT_DNS_USE_GLOBAL_CACHE;
pub use crate::curl_h::CURLOPT_EGDSOCKET;
pub use crate::curl_h::CURLOPT_ERRORBUFFER;
pub use crate::curl_h::CURLOPT_EXPECT_100_TIMEOUT_MS;
pub use crate::curl_h::CURLOPT_FAILONERROR;
pub use crate::curl_h::CURLOPT_FILETIME;
pub use crate::curl_h::CURLOPT_FNMATCH_DATA;
pub use crate::curl_h::CURLOPT_FNMATCH_FUNCTION;
pub use crate::curl_h::CURLOPT_FOLLOWLOCATION;
pub use crate::curl_h::CURLOPT_FORBID_REUSE;
pub use crate::curl_h::CURLOPT_FRESH_CONNECT;
pub use crate::curl_h::CURLOPT_FTPPORT;
pub use crate::curl_h::CURLOPT_FTPSSLAUTH;
pub use crate::curl_h::CURLOPT_FTP_ACCOUNT;
pub use crate::curl_h::CURLOPT_FTP_ALTERNATIVE_TO_USER;
pub use crate::curl_h::CURLOPT_FTP_CREATE_MISSING_DIRS;
pub use crate::curl_h::CURLOPT_FTP_FILEMETHOD;
pub use crate::curl_h::CURLOPT_FTP_RESPONSE_TIMEOUT;
pub use crate::curl_h::CURLOPT_FTP_SKIP_PASV_IP;
pub use crate::curl_h::CURLOPT_FTP_SSL_CCC;
pub use crate::curl_h::CURLOPT_FTP_USE_EPRT;
pub use crate::curl_h::CURLOPT_FTP_USE_EPSV;
pub use crate::curl_h::CURLOPT_FTP_USE_PRET;
pub use crate::curl_h::CURLOPT_GSSAPI_DELEGATION;
pub use crate::curl_h::CURLOPT_HEADER;
pub use crate::curl_h::CURLOPT_HEADERDATA;
pub use crate::curl_h::CURLOPT_HEADERFUNCTION;
pub use crate::curl_h::CURLOPT_HEADEROPT;
pub use crate::curl_h::CURLOPT_HTTP200ALIASES;
pub use crate::curl_h::CURLOPT_HTTPAUTH;
pub use crate::curl_h::CURLOPT_HTTPGET;
pub use crate::curl_h::CURLOPT_HTTPHEADER;
pub use crate::curl_h::CURLOPT_HTTPPOST;
pub use crate::curl_h::CURLOPT_HTTPPROXYTUNNEL;
pub use crate::curl_h::CURLOPT_HTTP_CONTENT_DECODING;
pub use crate::curl_h::CURLOPT_HTTP_TRANSFER_DECODING;
pub use crate::curl_h::CURLOPT_HTTP_VERSION;
pub use crate::curl_h::CURLOPT_IGNORE_CONTENT_LENGTH;
pub use crate::curl_h::CURLOPT_INFILESIZE;
pub use crate::curl_h::CURLOPT_INFILESIZE_LARGE;
pub use crate::curl_h::CURLOPT_INTERFACE;
pub use crate::curl_h::CURLOPT_INTERLEAVEDATA;
pub use crate::curl_h::CURLOPT_INTERLEAVEFUNCTION;
pub use crate::curl_h::CURLOPT_IOCTLDATA;
pub use crate::curl_h::CURLOPT_IOCTLFUNCTION;
pub use crate::curl_h::CURLOPT_IPRESOLVE;
pub use crate::curl_h::CURLOPT_ISSUERCERT;
pub use crate::curl_h::CURLOPT_KEEP_SENDING_ON_ERROR;
pub use crate::curl_h::CURLOPT_KEYPASSWD;
pub use crate::curl_h::CURLOPT_KRBLEVEL;
pub use crate::curl_h::CURLOPT_LASTENTRY;
pub use crate::curl_h::CURLOPT_LOCALPORT;
pub use crate::curl_h::CURLOPT_LOCALPORTRANGE;
pub use crate::curl_h::CURLOPT_LOGIN_OPTIONS;
pub use crate::curl_h::CURLOPT_LOW_SPEED_LIMIT;
pub use crate::curl_h::CURLOPT_LOW_SPEED_TIME;
pub use crate::curl_h::CURLOPT_MAIL_AUTH;
pub use crate::curl_h::CURLOPT_MAIL_FROM;
pub use crate::curl_h::CURLOPT_MAIL_RCPT;
pub use crate::curl_h::CURLOPT_MAXCONNECTS;
pub use crate::curl_h::CURLOPT_MAXFILESIZE;
pub use crate::curl_h::CURLOPT_MAXFILESIZE_LARGE;
pub use crate::curl_h::CURLOPT_MAXREDIRS;
pub use crate::curl_h::CURLOPT_MAX_RECV_SPEED_LARGE;
pub use crate::curl_h::CURLOPT_MAX_SEND_SPEED_LARGE;
pub use crate::curl_h::CURLOPT_NETRC;
pub use crate::curl_h::CURLOPT_NETRC_FILE;
pub use crate::curl_h::CURLOPT_NEW_DIRECTORY_PERMS;
pub use crate::curl_h::CURLOPT_NEW_FILE_PERMS;
pub use crate::curl_h::CURLOPT_NOBODY;
pub use crate::curl_h::CURLOPT_NOPROGRESS;
pub use crate::curl_h::CURLOPT_NOPROXY;
pub use crate::curl_h::CURLOPT_NOSIGNAL;
pub use crate::curl_h::CURLOPT_OBSOLETE40;
pub use crate::curl_h::CURLOPT_OBSOLETE72;
pub use crate::curl_h::CURLOPT_OPENSOCKETDATA;
pub use crate::curl_h::CURLOPT_OPENSOCKETFUNCTION;
pub use crate::curl_h::CURLOPT_PASSWORD;
pub use crate::curl_h::CURLOPT_PATH_AS_IS;
pub use crate::curl_h::CURLOPT_PINNEDPUBLICKEY;
pub use crate::curl_h::CURLOPT_PIPEWAIT;
pub use crate::curl_h::CURLOPT_PORT;
pub use crate::curl_h::CURLOPT_POST;
pub use crate::curl_h::CURLOPT_POSTFIELDS;
pub use crate::curl_h::CURLOPT_POSTFIELDSIZE;
pub use crate::curl_h::CURLOPT_POSTFIELDSIZE_LARGE;
pub use crate::curl_h::CURLOPT_POSTQUOTE;
pub use crate::curl_h::CURLOPT_POSTREDIR;
pub use crate::curl_h::CURLOPT_PREQUOTE;
pub use crate::curl_h::CURLOPT_PRE_PROXY;
pub use crate::curl_h::CURLOPT_PRIVATE;
pub use crate::curl_h::CURLOPT_PROGRESSDATA;
pub use crate::curl_h::CURLOPT_PROGRESSFUNCTION;
pub use crate::curl_h::CURLOPT_PROTOCOLS;
pub use crate::curl_h::CURLOPT_PROXY;
pub use crate::curl_h::CURLOPT_PROXYAUTH;
pub use crate::curl_h::CURLOPT_PROXYHEADER;
pub use crate::curl_h::CURLOPT_PROXYPASSWORD;
pub use crate::curl_h::CURLOPT_PROXYPORT;
pub use crate::curl_h::CURLOPT_PROXYTYPE;
pub use crate::curl_h::CURLOPT_PROXYUSERNAME;
pub use crate::curl_h::CURLOPT_PROXYUSERPWD;
pub use crate::curl_h::CURLOPT_PROXY_CAINFO;
pub use crate::curl_h::CURLOPT_PROXY_CAPATH;
pub use crate::curl_h::CURLOPT_PROXY_CRLFILE;
pub use crate::curl_h::CURLOPT_PROXY_KEYPASSWD;
pub use crate::curl_h::CURLOPT_PROXY_PINNEDPUBLICKEY;
pub use crate::curl_h::CURLOPT_PROXY_SERVICE_NAME;
pub use crate::curl_h::CURLOPT_PROXY_SSLCERT;
pub use crate::curl_h::CURLOPT_PROXY_SSLCERTTYPE;
pub use crate::curl_h::CURLOPT_PROXY_SSLKEY;
pub use crate::curl_h::CURLOPT_PROXY_SSLKEYTYPE;
pub use crate::curl_h::CURLOPT_PROXY_SSLVERSION;
pub use crate::curl_h::CURLOPT_PROXY_SSL_CIPHER_LIST;
pub use crate::curl_h::CURLOPT_PROXY_SSL_OPTIONS;
pub use crate::curl_h::CURLOPT_PROXY_SSL_VERIFYHOST;
pub use crate::curl_h::CURLOPT_PROXY_SSL_VERIFYPEER;
pub use crate::curl_h::CURLOPT_PROXY_TLSAUTH_PASSWORD;
pub use crate::curl_h::CURLOPT_PROXY_TLSAUTH_TYPE;
pub use crate::curl_h::CURLOPT_PROXY_TLSAUTH_USERNAME;
pub use crate::curl_h::CURLOPT_PROXY_TRANSFER_MODE;
pub use crate::curl_h::CURLOPT_PUT;
pub use crate::curl_h::CURLOPT_QUOTE;
pub use crate::curl_h::CURLOPT_RANDOM_FILE;
pub use crate::curl_h::CURLOPT_RANGE;
pub use crate::curl_h::CURLOPT_READDATA;
pub use crate::curl_h::CURLOPT_READFUNCTION;
pub use crate::curl_h::CURLOPT_REDIR_PROTOCOLS;
pub use crate::curl_h::CURLOPT_REFERER;
pub use crate::curl_h::CURLOPT_RESOLVE;
pub use crate::curl_h::CURLOPT_RESUME_FROM;
pub use crate::curl_h::CURLOPT_RESUME_FROM_LARGE;
pub use crate::curl_h::CURLOPT_RTSP_CLIENT_CSEQ;
pub use crate::curl_h::CURLOPT_RTSP_REQUEST;
pub use crate::curl_h::CURLOPT_RTSP_SERVER_CSEQ;
pub use crate::curl_h::CURLOPT_RTSP_SESSION_ID;
pub use crate::curl_h::CURLOPT_RTSP_STREAM_URI;
pub use crate::curl_h::CURLOPT_RTSP_TRANSPORT;
pub use crate::curl_h::CURLOPT_SASL_IR;
pub use crate::curl_h::CURLOPT_SEEKDATA;
pub use crate::curl_h::CURLOPT_SEEKFUNCTION;
pub use crate::curl_h::CURLOPT_SERVICE_NAME;
pub use crate::curl_h::CURLOPT_SHARE;
pub use crate::curl_h::CURLOPT_SOCKOPTDATA;
pub use crate::curl_h::CURLOPT_SOCKOPTFUNCTION;
pub use crate::curl_h::CURLOPT_SOCKS5_GSSAPI_NEC;
pub use crate::curl_h::CURLOPT_SOCKS5_GSSAPI_SERVICE;
pub use crate::curl_h::CURLOPT_SSH_AUTH_TYPES;
pub use crate::curl_h::CURLOPT_SSH_HOST_PUBLIC_KEY_MD5;
pub use crate::curl_h::CURLOPT_SSH_KEYDATA;
pub use crate::curl_h::CURLOPT_SSH_KEYFUNCTION;
pub use crate::curl_h::CURLOPT_SSH_KNOWNHOSTS;
pub use crate::curl_h::CURLOPT_SSH_PRIVATE_KEYFILE;
pub use crate::curl_h::CURLOPT_SSH_PUBLIC_KEYFILE;
pub use crate::curl_h::CURLOPT_SSLCERT;
pub use crate::curl_h::CURLOPT_SSLCERTTYPE;
pub use crate::curl_h::CURLOPT_SSLENGINE;
pub use crate::curl_h::CURLOPT_SSLENGINE_DEFAULT;
pub use crate::curl_h::CURLOPT_SSLKEY;
pub use crate::curl_h::CURLOPT_SSLKEYTYPE;
pub use crate::curl_h::CURLOPT_SSLVERSION;
pub use crate::curl_h::CURLOPT_SSL_CIPHER_LIST;
pub use crate::curl_h::CURLOPT_SSL_CTX_DATA;
pub use crate::curl_h::CURLOPT_SSL_CTX_FUNCTION;
pub use crate::curl_h::CURLOPT_SSL_ENABLE_ALPN;
pub use crate::curl_h::CURLOPT_SSL_ENABLE_NPN;
pub use crate::curl_h::CURLOPT_SSL_FALSESTART;
pub use crate::curl_h::CURLOPT_SSL_OPTIONS;
pub use crate::curl_h::CURLOPT_SSL_SESSIONID_CACHE;
pub use crate::curl_h::CURLOPT_SSL_VERIFYHOST;
pub use crate::curl_h::CURLOPT_SSL_VERIFYPEER;
pub use crate::curl_h::CURLOPT_SSL_VERIFYSTATUS;
pub use crate::curl_h::CURLOPT_STDERR;
pub use crate::curl_h::CURLOPT_STREAM_DEPENDS;
pub use crate::curl_h::CURLOPT_STREAM_DEPENDS_E;
pub use crate::curl_h::CURLOPT_STREAM_WEIGHT;
pub use crate::curl_h::CURLOPT_SUPPRESS_CONNECT_HEADERS;
pub use crate::curl_h::CURLOPT_TCP_FASTOPEN;
pub use crate::curl_h::CURLOPT_TCP_KEEPALIVE;
pub use crate::curl_h::CURLOPT_TCP_KEEPIDLE;
pub use crate::curl_h::CURLOPT_TCP_KEEPINTVL;
pub use crate::curl_h::CURLOPT_TCP_NODELAY;
pub use crate::curl_h::CURLOPT_TELNETOPTIONS;
pub use crate::curl_h::CURLOPT_TFTP_BLKSIZE;
pub use crate::curl_h::CURLOPT_TFTP_NO_OPTIONS;
pub use crate::curl_h::CURLOPT_TIMECONDITION;
pub use crate::curl_h::CURLOPT_TIMEOUT;
pub use crate::curl_h::CURLOPT_TIMEOUT_MS;
pub use crate::curl_h::CURLOPT_TIMEVALUE;
pub use crate::curl_h::CURLOPT_TLSAUTH_PASSWORD;
pub use crate::curl_h::CURLOPT_TLSAUTH_TYPE;
pub use crate::curl_h::CURLOPT_TLSAUTH_USERNAME;
pub use crate::curl_h::CURLOPT_TRANSFERTEXT;
pub use crate::curl_h::CURLOPT_TRANSFER_ENCODING;
pub use crate::curl_h::CURLOPT_UNIX_SOCKET_PATH;
pub use crate::curl_h::CURLOPT_UNRESTRICTED_AUTH;
pub use crate::curl_h::CURLOPT_UPLOAD;
pub use crate::curl_h::CURLOPT_URL;
pub use crate::curl_h::CURLOPT_USERAGENT;
pub use crate::curl_h::CURLOPT_USERNAME;
pub use crate::curl_h::CURLOPT_USERPWD;
pub use crate::curl_h::CURLOPT_USE_SSL;
pub use crate::curl_h::CURLOPT_VERBOSE;
pub use crate::curl_h::CURLOPT_WILDCARDMATCH;
pub use crate::curl_h::CURLOPT_WRITEDATA;
pub use crate::curl_h::CURLOPT_WRITEFUNCTION;
pub use crate::curl_h::CURLOPT_XFERINFOFUNCTION;
pub use crate::curl_h::CURLOPT_XOAUTH2_BEARER;
pub use crate::curl_h::CURL_LAST;
pub use crate::multi_h::C2RustUnnamed_20;
pub use crate::multi_h::CURLMcode;
pub use crate::multi_h::CURLMsg;
pub use crate::multi_h::CURLM;
pub use crate::multi_h::CURLMSG;
pub use crate::multi_h::CURLMSG_DONE;
pub use crate::multi_h::CURLMSG_LAST;
pub use crate::multi_h::CURLMSG_NONE;
pub use crate::multi_h::CURLM_ADDED_ALREADY;
pub use crate::multi_h::CURLM_BAD_EASY_HANDLE;
pub use crate::multi_h::CURLM_BAD_HANDLE;
pub use crate::multi_h::CURLM_BAD_SOCKET;
pub use crate::multi_h::CURLM_CALL_MULTI_PERFORM;
pub use crate::multi_h::CURLM_INTERNAL_ERROR;
pub use crate::multi_h::CURLM_LAST;
pub use crate::multi_h::CURLM_OK;
pub use crate::multi_h::CURLM_OUT_OF_MEMORY;
pub use crate::multi_h::CURLM_UNKNOWN_OPTION;
pub use crate::stdlib::mbstate_t;

pub use crate::client_h::clientConnection_t;
pub use crate::client_h::clientStatic_t;
pub use crate::client_h::serverInfo_t;
pub use crate::src::client::cl_input::CL_WritePacket;
pub use crate::src::client::cl_main::clc;
pub use crate::src::client::cl_main::cls;
pub use crate::src::client::cl_main::CL_AddReliableCommand;
pub use crate::src::client::cl_main::CL_NextDownload;

/*
===========================================================================
Copyright (C) 2006 Tony J. White (tjw@tjw.org)

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
#[no_mangle]

pub static mut cl_cURLLib: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]

pub static mut qcurl_version: Option<unsafe extern "C" fn() -> *mut libc::c_char> = None;
#[no_mangle]

pub static mut qcurl_easy_init: Option<unsafe extern "C" fn() -> *mut libc::c_void> = None;
#[no_mangle]

pub static mut qcurl_easy_setopt: Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: CURLoption,
        _: ...
    ) -> CURLcode,
> = None;
#[no_mangle]

pub static mut qcurl_easy_perform: Option<
    unsafe extern "C" fn(_: *mut libc::c_void) -> CURLcode,
> = None;
#[no_mangle]

pub static mut qcurl_easy_cleanup: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()> = None;
#[no_mangle]

pub static mut qcurl_easy_getinfo: Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: CURLINFO,
        _: ...
    ) -> CURLcode,
> = None;
#[no_mangle]

pub static mut qcurl_easy_duphandle: Option<
    unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void,
> = None;
#[no_mangle]

pub static mut qcurl_easy_reset: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()> = None;
#[no_mangle]

pub static mut qcurl_easy_strerror: Option<
    unsafe extern "C" fn(_: CURLcode) -> *const libc::c_char,
> = None;
#[no_mangle]

pub static mut qcurl_multi_init: Option<unsafe extern "C" fn() -> *mut libc::c_void> = None;
#[no_mangle]

pub static mut qcurl_multi_add_handle: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> CURLMcode,
> = None;
#[no_mangle]

pub static mut qcurl_multi_remove_handle: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> CURLMcode,
> = None;
#[no_mangle]

pub static mut qcurl_multi_fdset: Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut fd_set,
        _: *mut i32,
    ) -> CURLMcode,
> = None;
#[no_mangle]

pub static mut qcurl_multi_perform: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut i32) -> CURLMcode,
> = None;
#[no_mangle]

pub static mut qcurl_multi_cleanup: Option<
    unsafe extern "C" fn(_: *mut libc::c_void) -> CURLMcode,
> = None;
#[no_mangle]

pub static mut qcurl_multi_info_read: Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut i32) -> *mut CURLMsg,
> = None;
#[no_mangle]

pub static mut qcurl_multi_strerror: Option<
    unsafe extern "C" fn(_: CURLMcode) -> *const libc::c_char,
> = None;

static mut cURLLib: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
/*
=================
GPA
=================
*/

unsafe extern "C" fn GPA(mut str: *mut libc::c_char) -> *mut libc::c_void {
    let mut rv: *mut libc::c_void = 0 as *mut libc::c_void;
    rv = crate::stdlib::SDL_LoadFunction(cURLLib, str);
    if rv.is_null() {
        Com_Printf(
            b"Can\'t load symbol %s\n\x00" as *const u8 as *const libc::c_char,
            str,
        );
        clc.cURLEnabled = qfalse;
        return 0 as *mut libc::c_void;
    } else {
        Com_DPrintf(
            b"Loaded symbol %s (0x%p)\n\x00" as *const u8 as *const libc::c_char,
            str,
            rv,
        );
        return rv;
    };
}
/* USE_CURL_DLOPEN */
/*
=================
CL_cURL_Init
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_cURL_Init() -> qboolean {
    if !cURLLib.is_null() {
        return qtrue;
    }
    Com_Printf(
        b"Loading \"%s\"...\x00" as *const u8 as *const libc::c_char,
        (*cl_cURLLib).string,
    );
    cURLLib = crate::src::sys::sys_main::Sys_LoadDll(
        (*cl_cURLLib).string,
        qtrue,
    );
    if cURLLib.is_null() {
        // On some linux distributions there is no libcurl.so.3, but only libcurl.so.4. That one works too.
        cURLLib = crate::src::sys::sys_main::Sys_LoadDll(
            b"libcurl.so.3\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        if cURLLib.is_null() {
            return qfalse;
        }
    }
    clc.cURLEnabled = qtrue;
    qcurl_version = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn() -> *mut libc::c_char>,
    >(GPA(
        b"curl_version\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    ));
    qcurl_easy_init = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(GPA(
        b"curl_easy_init\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_easy_setopt = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: CURLoption,
                _: ...
            ) -> CURLcode,
        >,
    >(GPA(
        b"curl_easy_setopt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_easy_perform = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CURLcode>,
    >(GPA(
        b"curl_easy_perform\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_easy_cleanup = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    >(GPA(
        b"curl_easy_cleanup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_easy_getinfo = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: CURLINFO,
                _: ...
            ) -> CURLcode,
        >,
    >(GPA(
        b"curl_easy_getinfo\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_easy_duphandle = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
    >(GPA(
        b"curl_easy_duphandle\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_easy_reset = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    >(GPA(
        b"curl_easy_reset\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_easy_strerror = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: CURLcode) -> *const libc::c_char>,
    >(GPA(
        b"curl_easy_strerror\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_multi_init = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    >(GPA(
        b"curl_multi_init\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_multi_add_handle = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *mut libc::c_void,
            ) -> CURLMcode,
        >,
    >(GPA(
        b"curl_multi_add_handle\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    ));
    qcurl_multi_remove_handle = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *mut libc::c_void,
            ) -> CURLMcode,
        >,
    >(GPA(
        b"curl_multi_remove_handle\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    ));
    qcurl_multi_fdset = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *mut fd_set,
                _: *mut fd_set,
                _: *mut fd_set,
                _: *mut i32,
            ) -> CURLMcode,
        >,
    >(GPA(
        b"curl_multi_fdset\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_multi_perform = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<
            unsafe extern "C" fn(_: *mut libc::c_void, _: *mut i32) -> CURLMcode,
        >,
    >(GPA(
        b"curl_multi_perform\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_multi_cleanup = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CURLMcode>,
    >(GPA(
        b"curl_multi_cleanup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    qcurl_multi_info_read = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<
            unsafe extern "C" fn(_: *mut libc::c_void, _: *mut i32) -> *mut CURLMsg,
        >,
    >(GPA(
        b"curl_multi_info_read\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    ));
    qcurl_multi_strerror = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: CURLMcode) -> *const libc::c_char>,
    >(GPA(
        b"curl_multi_strerror\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ));
    if clc.cURLEnabled as u64 == 0 {
        CL_cURL_Shutdown();
        Com_Printf(
            b"FAIL One or more symbols not found\n\x00" as *const u8 as *const libc::c_char,
        );
        return qfalse;
    }
    Com_Printf(b"OK\n\x00" as *const u8 as *const libc::c_char);
    return qtrue;
    /* USE_CURL_DLOPEN */
}
/*
=================
CL_cURL_Shutdown
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_cURL_Shutdown() {
    CL_cURL_Cleanup();
    if !cURLLib.is_null() {
        crate::stdlib::SDL_UnloadObject(cURLLib);
        cURLLib = 0 as *mut libc::c_void
    }
    qcurl_easy_init = None;
    qcurl_easy_setopt = None;
    qcurl_easy_perform = None;
    qcurl_easy_cleanup = None;
    qcurl_easy_getinfo = None;
    qcurl_easy_duphandle = None;
    qcurl_easy_reset = None;
    qcurl_multi_init = None;
    qcurl_multi_add_handle = None;
    qcurl_multi_remove_handle = None;
    qcurl_multi_fdset = None;
    qcurl_multi_perform = None;
    qcurl_multi_cleanup = None;
    qcurl_multi_info_read = None;
    qcurl_multi_strerror = None;
    /* USE_CURL_DLOPEN */
}
#[no_mangle]

pub unsafe extern "C" fn CL_cURL_Cleanup() {
    if !clc.downloadCURLM.is_null() {
        let mut result: CURLMcode = CURLM_OK;
        if !clc.downloadCURL.is_null() {
            result = qcurl_multi_remove_handle.expect("non-null function pointer")(
                clc.downloadCURLM,
                clc.downloadCURL,
            );
            if result as i32 != CURLM_OK as i32 {
                Com_DPrintf(
                    b"qcurl_multi_remove_handle failed: %s\n\x00" as *const u8
                        as *const libc::c_char,
                    qcurl_multi_strerror.expect("non-null function pointer")(result),
                );
            }
            qcurl_easy_cleanup.expect("non-null function pointer")(
                clc.downloadCURL,
            );
        }
        result = qcurl_multi_cleanup.expect("non-null function pointer")(
            clc.downloadCURLM,
        );
        if result as i32 != CURLM_OK as i32 {
            Com_DPrintf(
                b"CL_cURL_Cleanup: qcurl_multi_cleanup failed: %s\n\x00" as *const u8
                    as *const libc::c_char,
                qcurl_multi_strerror.expect("non-null function pointer")(result),
            );
        }
        clc.downloadCURLM = 0 as *mut libc::c_void;
        clc.downloadCURL = 0 as *mut libc::c_void
    } else if !clc.downloadCURL.is_null() {
        qcurl_easy_cleanup.expect("non-null function pointer")(
            clc.downloadCURL,
        );
        clc.downloadCURL = 0 as *mut libc::c_void
    };
}

unsafe extern "C" fn CL_cURL_CallbackProgress(
    mut _dummy: *mut libc::c_void,
    mut dltotal: f64,
    mut dlnow: f64,
    mut _ultotal: f64,
    mut _ulnow: f64,
) -> i32 {
    clc.downloadSize = dltotal as i32;
    Cvar_SetValue(
        b"cl_downloadSize\x00" as *const u8 as *const libc::c_char,
        clc.downloadSize as f32,
    );
    clc.downloadCount = dlnow as i32;
    Cvar_SetValue(
        b"cl_downloadCount\x00" as *const u8 as *const libc::c_char,
        clc.downloadCount as f32,
    );
    return 0 as i32;
}

unsafe extern "C" fn CL_cURL_CallbackWrite(
    mut buffer: *mut libc::c_void,
    mut size: size_t,
    mut nmemb: size_t,
    mut stream: *mut libc::c_void,
) -> size_t {
    FS_Write(
        buffer,
        size.wrapping_mul(nmemb) as i32,
        *(stream as *mut fileHandle_t).offset(0 as i32 as isize),
    );
    return size.wrapping_mul(nmemb);
}
#[no_mangle]

pub unsafe extern "C" fn qcurl_easy_setopt_warn(
    mut curl: *mut libc::c_void,
    mut option: CURLoption,
    mut args: ...
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut argp: ::std::ffi::VaListImpl;
    argp = args.clone();
    if (option as u32) < 10000 as i32 as u32 {
        let mut longValue: isize = argp.as_va_list().arg::<isize>();
        result = qcurl_easy_setopt.expect("non-null function pointer")(curl, option, longValue)
    } else if (option as u32) < 30000 as i32 as u32 {
        let mut pointerValue: *mut libc::c_void = argp.as_va_list().arg::<*mut libc::c_void>();
        result = qcurl_easy_setopt.expect("non-null function pointer")(curl, option, pointerValue)
    } else {
        let mut offsetValue: curl_off_t =
            argp.as_va_list().arg::<curl_off_t>();
        result = qcurl_easy_setopt.expect("non-null function pointer")(curl, option, offsetValue)
    }
    if result as u32 != CURLE_OK as i32 as u32 {
        Com_DPrintf(
            b"qcurl_easy_setopt failed: %s\n\x00" as *const u8 as *const libc::c_char,
            qcurl_easy_strerror.expect("non-null function pointer")(result),
        );
    }
    return result;
}
#[no_mangle]

pub unsafe extern "C" fn CL_cURL_BeginDownload(
    mut localName: *const libc::c_char,
    mut remoteURL: *const libc::c_char,
) {
    let mut result: CURLMcode = CURLM_OK;
    clc.cURLUsed = qtrue;
    Com_Printf(
        b"URL: %s\n\x00" as *const u8 as *const libc::c_char,
        remoteURL,
    );
    Com_DPrintf(b"***** CL_cURL_BeginDownload *****\nLocalname: %s\nRemoteURL: %s\n****************************\n\x00"
                    as *const u8 as *const libc::c_char, localName,
                remoteURL);
    CL_cURL_Cleanup();
    Q_strncpyz(
        clc.downloadURL.as_mut_ptr(),
        remoteURL,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    Q_strncpyz(
        clc.downloadName.as_mut_ptr(),
        localName,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    Com_sprintf(
        clc
            .downloadTempName
            .as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
        b"%s.tmp\x00" as *const u8 as *const libc::c_char,
        localName,
    );
    // Set so UI gets access to it
    Cvar_Set(
        b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
        localName,
    ); // Starting new file
    Cvar_Set(
        b"cl_downloadSize\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    Cvar_Set(
        b"cl_downloadCount\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    Cvar_SetValue(
        b"cl_downloadTime\x00" as *const u8 as *const libc::c_char,
        cls.realtime as f32,
    );
    clc.downloadBlock = 0 as i32;
    clc.downloadCount = 0 as i32;
    clc.downloadCURL =
        qcurl_easy_init.expect("non-null function pointer")();
    if clc.downloadCURL.is_null() {
        Com_Error(
            ERR_DROP as i32,
            b"CL_cURL_BeginDownload: qcurl_easy_init() failed\x00" as *const u8
                as *const libc::c_char,
        );
    }
    clc.download = FS_SV_FOpenFileWrite(
        clc
            .downloadTempName
            .as_mut_ptr(),
    );
    if clc.download == 0 {
        Com_Error(
            ERR_DROP as i32,
            b"CL_cURL_BeginDownload: failed to open %s for writing\x00" as *const u8
                as *const libc::c_char,
            clc
                .downloadTempName
                .as_mut_ptr(),
        );
    }
    if (*com_developer).integer != 0 {
        qcurl_easy_setopt_warn(
            clc.downloadCURL,
            CURLOPT_VERBOSE,
            1 as i32,
        );
    }
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_URL,
        clc.downloadURL.as_mut_ptr(),
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_TRANSFERTEXT,
        0 as i32,
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_REFERER,
        va(
            b"ioQ3://%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            NET_AdrToString(
                clc.serverAddress as netadr_t,
            ),
        ),
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_USERAGENT,
        va(
            b"%s %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"ioq3 1.36_GIT_d0fe4462-2020-01-10\x00" as *const u8 as *const libc::c_char,
            qcurl_version.expect("non-null function pointer")(),
        ),
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_WRITEFUNCTION,
        Some(
            CL_cURL_CallbackWrite
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: size_t,
                    _: size_t,
                    _: *mut libc::c_void,
                ) -> size_t,
        ),
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_WRITEDATA,
        &mut clc.download
            as *mut fileHandle_t,
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_NOPROGRESS,
        0 as i32,
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_PROGRESSFUNCTION,
        Some(
            CL_cURL_CallbackProgress
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: f64,
                    _: f64,
                    _: f64,
                    _: f64,
                ) -> i32,
        ),
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_PROGRESSDATA,
        0 as *mut libc::c_void,
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_FAILONERROR,
        1 as i32,
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_FOLLOWLOCATION,
        1 as i32,
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_MAXREDIRS,
        5 as i32,
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_PROTOCOLS,
        (1 as i32) << 0 as i32
            | (1 as i32) << 1 as i32
            | (1 as i32) << 2 as i32
            | (1 as i32) << 3 as i32,
    );
    qcurl_easy_setopt_warn(
        clc.downloadCURL,
        CURLOPT_BUFFERSIZE,
        524288 as i32,
    );
    clc.downloadCURLM =
        qcurl_multi_init.expect("non-null function pointer")();
    if clc.downloadCURLM.is_null() {
        qcurl_easy_cleanup.expect("non-null function pointer")(
            clc.downloadCURL,
        );
        clc.downloadCURL = 0 as *mut libc::c_void;
        Com_Error(
            ERR_DROP as i32,
            b"CL_cURL_BeginDownload: qcurl_multi_init() failed\x00" as *const u8
                as *const libc::c_char,
        );
    }
    result = qcurl_multi_add_handle.expect("non-null function pointer")(
        clc.downloadCURLM,
        clc.downloadCURL,
    );
    if result as i32 != CURLM_OK as i32 {
        qcurl_easy_cleanup.expect("non-null function pointer")(
            clc.downloadCURL,
        );
        clc.downloadCURL = 0 as *mut libc::c_void;
        Com_Error(
            ERR_DROP as i32,
            b"CL_cURL_BeginDownload: qcurl_multi_add_handle() failed: %s\x00" as *const u8
                as *const libc::c_char,
            qcurl_multi_strerror.expect("non-null function pointer")(result),
        );
    }
    if clc.sv_allowDownload & 8 as i32 == 0
        && clc.cURLDisconnected as u64 == 0
    {
        CL_AddReliableCommand(
            b"disconnect\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        CL_WritePacket();
        CL_WritePacket();
        CL_WritePacket();
        clc.cURLDisconnected = qtrue
    };
}
/*
===========================================================================
Copyright (C) 2006 Tony J. White (tjw@tjw.org)

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn CL_cURL_PerformDownload() {
    let mut res: CURLMcode = CURLM_OK;
    let mut msg: *mut CURLMsg = 0 as *mut CURLMsg;
    let mut c: i32 = 0;
    let mut i: i32 = 0 as i32;
    res = qcurl_multi_perform.expect("non-null function pointer")(
        clc.downloadCURLM,
        &mut c,
    );
    while res as i32 == CURLM_CALL_MULTI_PERFORM as i32 && i < 100 as i32 {
        res = qcurl_multi_perform.expect("non-null function pointer")(
            clc.downloadCURLM,
            &mut c,
        );
        i += 1
    }
    if res as i32 == CURLM_CALL_MULTI_PERFORM as i32 {
        return;
    }
    msg = qcurl_multi_info_read.expect("non-null function pointer")(
        clc.downloadCURLM,
        &mut c,
    );
    if msg.is_null() {
        return;
    }
    FS_FCloseFile(clc.download);
    if (*msg).msg as u32 == CURLMSG_DONE as i32 as u32
        && (*msg).data.result as u32 == CURLE_OK as i32 as u32
    {
        FS_SV_Rename(
            clc
                .downloadTempName
                .as_mut_ptr(),
            clc.downloadName.as_mut_ptr(),
            qfalse,
        );
        clc.downloadRestart = qtrue
    } else {
        let mut code: isize = 0;
        qcurl_easy_getinfo.expect("non-null function pointer")(
            (*msg).easy_handle,
            CURLINFO_RESPONSE_CODE,
            &mut code as *mut isize,
        );
        Com_Error(
            ERR_DROP as i32,
            b"Download Error: %s Code: %ld URL: %s\x00" as *const u8 as *const libc::c_char,
            qcurl_easy_strerror.expect("non-null function pointer")((*msg).data.result),
            code,
            clc.downloadURL.as_mut_ptr(),
        );
    }
    CL_NextDownload();
}
/* USE_CURL */
