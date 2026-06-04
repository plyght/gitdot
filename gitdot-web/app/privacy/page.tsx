import type { Metadata } from "next";
import { League_Spartan } from "next/font/google";
import Link from "@/ui/link";

const leagueSpartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
});

export const metadata: Metadata = {
  title: "Privacy · gitdot",
  description: "We do not sell your data or train on your code.",
};

export default function Privacy() {
  return (
    <div
      className={`${leagueSpartan.className} h-screen w-full overflow-y-auto`}
    >
      <main className="mx-auto flex max-w-2xl flex-col gap-6 px-4 py-12">
        <header>
          <h1 className="font-bold text-2xl">Privacy Policy</h1>
          <p className="text-muted-foreground text-sm">
            Operated by Async Inc. · Last updated June 4, 2026
          </p>
        </header>

        <p>
          The short version: we do not sell your data, and we do not train on
          your code. Ever.
        </p>

        <div>
          <h2 className="font-bold text-lg">What we collect</h2>
          <ul>
            <li>
              • Account information you give us: your email, username, and
              profile details.
            </li>
            <li>
              • The repositories, code, and content you choose to store on
              gitdot.
            </li>
            <li>
              • Anonymous analytics, such as page views, that are never tied to
              your identity.
            </li>
          </ul>
        </div>

        <div>
          <h2 className="font-bold text-lg">How we use it</h2>
          <p>
            Only to operate, secure, and improve gitdot: to run your
            repositories, sign you in, and keep the service working. <br />
          </p>
        </div>

        <div>
          <h2 className="font-bold text-lg">Cookies</h2>
          <p>
            We use only essential cookies: to keep you signed in and to make
            gitdot faster. <br />
            We do not use cookies for advertising or cross-site tracking.
          </p>
        </div>

        <div>
          <h2 className="font-bold text-lg">What we will never do</h2>
          <ul className="mb-2">
            <li>• We will never sell your data.</li>
            <li>• We will never train AI models on your code.</li>
            <li>
              • We will never share your private repositories or personal
              information with anyone for their own use.
            </li>
          </ul>
          <p>
            We are building gitdot to last, not to sell. But in the unlikely
            event it is ever acquired, the buyer is bound by this policy as
            written, or we will notify you first, so you can export or delete
            your data before anything changes.
          </p>
        </div>

        <div>
          <h2 className="font-bold text-lg">Service providers</h2>
          <p>
            We rely on a small number of infrastructure providers to run gitdot:
            Google Cloud Platform, Vercel, Resend, and Cloudflare. They process
            data only on our behalf and only to provide the service, never for
            their own purposes.
          </p>
        </div>

        <div>
          <h2 className="font-bold text-lg">Your choices</h2>
          <p>
            To delete or export your account and data, please{" "}
            <Link href="mailto:founders@gitdot.io" className="underline">
              email us
            </Link>
            . <br />
            Deleting your account removes your personal information and
            repositories from our systems.
          </p>
        </div>

        <div>
          <h2 className="font-bold text-lg">Changes</h2>
          <p>
            If we update this policy, we will post the changes here and update
            the date above. <br />
            We will never change it to weaken the promises above.
          </p>
        </div>

        <div>
          <h2 className="font-bold text-lg">Contact</h2>
          <p>
            Questions? Email us at{" "}
            <Link href="mailto:founders@gitdot.io" className="underline">
              founders@gitdot.io
            </Link>
            .
          </p>
        </div>
      </main>
    </div>
  );
}
