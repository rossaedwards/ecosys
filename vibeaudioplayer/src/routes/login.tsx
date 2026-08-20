import { createFileRoute, Link } from "@tanstack/react-router";
import { GROK_PROVIDERS, authEnabled, signIn } from "@/lib/auth/client";
import { Button } from "@/components/ui/button";

export const Route = createFileRoute("/login")({ component: Login });

function Login() {
  return (
    <main className="relative grid min-h-dvh place-items-center overflow-hidden bg-bg px-6 text-fg">
      <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(ellipse_at_top,rgba(75,0,130,0.35),transparent_55%),radial-gradient(ellipse_at_bottom,rgba(0,128,128,0.22),transparent_50%)]" />
      <div className="relative w-full max-w-sm rounded-[28px] bg-surface/80 p-6 shadow-sheet backdrop-blur-xl">
        <p className="text-[11px] font-medium uppercase tracking-[0.2em] text-accent">VASP 3.69</p>
        <h1 className="mt-1 font-display text-2xl font-semibold tracking-tight">Sign in</h1>
        <p className="mt-2 text-pretty text-sm leading-relaxed text-muted">
          Optional. Local playback and the visualizer work without an account.
        </p>
        <div className="mt-5 flex flex-col gap-2">
          {authEnabled ? (
            GROK_PROVIDERS.map((p) => (
              <Button
                key={p.providerId}
                type="button"
                variant="secondary"
                className="w-full"
                onClick={() => void signIn(p.providerId, { callbackURL: "/" })}
              >
                Continue with {p.label}
              </Button>
            ))
          ) : (
            <p className="text-sm text-muted">Sign-in is disabled.</p>
          )}
        </div>
        <Link
          to="/"
          className="mt-5 block text-center text-sm text-muted underline-offset-4 hover:text-fg hover:underline"
        >
          Back to player
        </Link>
      </div>
    </main>
  );
}
