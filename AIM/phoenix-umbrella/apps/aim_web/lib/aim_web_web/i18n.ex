defmodule AimWeb.I18n do
  @moduledoc """
  Lightweight i18n. 6 UN languages + Georgian.

  - en (English)
  - fr (French)
  - es (Spanish)
  - ru (Russian)
  - zh (Chinese, Simplified)
  - ar (Arabic, RTL)
  - ka (Georgian)
  """

  @default :en
  @locales [:en, :fr, :es, :ru, :zh, :ar, :ka]

  @rtl ~w(ar)

  @names %{
    en: "English",
    fr: "Français",
    es: "Español",
    ru: "Русский",
    zh: "中文",
    ar: "العربية",
    ka: "ქართული"
  }

  @strings %{
    "app.title" => %{
      en: "AIM",
      fr: "AIM",
      es: "AIM",
      ru: "AIM",
      zh: "AIM",
      ar: "AIM",
      ka: "AIM"
    },
    "nav.home" => %{
      en: "Home",
      fr: "Accueil",
      es: "Inicio",
      ru: "Главная",
      zh: "首页",
      ar: "الرئيسية",
      ka: "მთავარი"
    },
    "nav.chat" => %{
      en: "Chat",
      fr: "Discuter",
      es: "Chat",
      ru: "Чат",
      zh: "聊天",
      ar: "محادثة",
      ka: "ჩატი"
    },
    "nav.intake" => %{
      en: "Intake",
      fr: "Admission",
      es: "Admisión",
      ru: "Приём",
      zh: "接诊",
      ar: "الاستقبال",
      ka: "მიღება"
    },
    "nav.cases" => %{
      en: "Cases",
      fr: "Cas",
      es: "Casos",
      ru: "Случаи",
      zh: "病例",
      ar: "الحالات",
      ka: "შემთხვევები"
    },
    "nav.consult" => %{
      en: "Consultation",
      fr: "Consultation",
      es: "Consulta",
      ru: "Консультация",
      zh: "咨询",
      ar: "استشارة",
      ka: "კონსულტაცია"
    },
    "home.heading" => %{
      en: "AIM — medical diagnostics platform",
      fr: "AIM — plateforme de diagnostic médical",
      es: "AIM — plataforma de diagnóstico médico",
      ru: "AIM — система медицинской диагностики",
      zh: "AIM — 医疗诊断平台",
      ar: "AIM — منصة التشخيص الطبي",
      ka: "AIM — სამედიცინო დიაგნოსტიკის სისტემა"
    },
    "home.tagline" => %{
      en: "Phoenix umbrella + Rust core. Skeleton.",
      fr: "Phoenix umbrella + cœur Rust. Squelette.",
      es: "Phoenix umbrella + núcleo Rust. Esqueleto.",
      ru: "Phoenix umbrella + ядро на Rust. Скелет.",
      zh: "Phoenix umbrella + Rust 核心。骨架。",
      ar: "Phoenix umbrella + نواة Rust. هيكل.",
      ka: "Phoenix umbrella + Rust ბირთვი. ჩონჩხი."
    },
    "chat.heading" => %{
      en: "Chat",
      fr: "Discussion",
      es: "Chat",
      ru: "Чат",
      zh: "聊天",
      ar: "محادثة",
      ka: "ჩატი"
    },
    "chat.todo" => %{
      en: "TODO: connect to AimOrchestrator.chat/2 and aim-llm :8770.",
      fr: "TODO : connecter à AimOrchestrator.chat/2 et aim-llm :8770.",
      es: "TODO: conectar a AimOrchestrator.chat/2 y aim-llm :8770.",
      ru: "TODO: подключить к AimOrchestrator.chat/2 и aim-llm :8770.",
      zh: "TODO：连接到 AimOrchestrator.chat/2 和 aim-llm :8770。",
      ar: "TODO: ربط بـ AimOrchestrator.chat/2 و aim-llm :8770.",
      ka: "TODO: დაკავშირება AimOrchestrator.chat/2-სა და aim-llm :8770-სთან."
    },
    "intake.heading" => %{
      en: "Patient intake",
      fr: "Admission du patient",
      es: "Admisión del paciente",
      ru: "Приём пациента",
      zh: "患者接诊",
      ar: "استقبال المريض",
      ka: "პაციენტის მიღება"
    },
    "cases.heading" => %{
      en: "Case list",
      fr: "Liste des cas",
      es: "Lista de casos",
      ru: "Список случаев",
      zh: "病例列表",
      ar: "قائمة الحالات",
      ka: "შემთხვევების სია"
    },
    "case.heading" => %{
      en: "Case",
      fr: "Cas",
      es: "Caso",
      ru: "Случай",
      zh: "病例",
      ar: "حالة",
      ka: "შემთხვევა"
    },
    "common.todo" => %{
      en: "TODO",
      fr: "À FAIRE",
      es: "PENDIENTE",
      ru: "TODO",
      zh: "待办",
      ar: "قيد الإنجاز",
      ka: "TODO"
    },
    "lang.label" => %{
      en: "Language",
      fr: "Langue",
      es: "Idioma",
      ru: "Язык",
      zh: "语言",
      ar: "اللغة",
      ka: "ენა"
    }
  }

  def default, do: @default
  def locales, do: @locales
  def names, do: @names
  def name(locale), do: Map.get(@names, locale, to_string(locale))
  def rtl?(locale), do: to_string(locale) in @rtl

  @doc "Translate `key` to `locale`, falling back to default. Unknown key → key itself."
  def t(key, locale) when is_atom(locale) do
    case Map.get(@strings, key) do
      nil ->
        key

      translations ->
        Map.get(translations, locale) ||
          Map.get(translations, @default) ||
          key
    end
  end

  def t(key, locale) when is_binary(locale) do
    t(key, parse(locale))
  end

  @doc "Coerce arbitrary input (string/atom) to a supported locale, default if unknown."
  def parse(nil), do: @default
  def parse(loc) when is_atom(loc), do: if(loc in @locales, do: loc, else: @default)

  def parse(loc) when is_binary(loc) do
    a = String.downcase(loc) |> String.split("-") |> hd() |> safe_atom()
    if a in @locales, do: a, else: @default
  end

  defp safe_atom(s) do
    try do
      String.to_existing_atom(s)
    rescue
      ArgumentError -> @default
    end
  end
end
