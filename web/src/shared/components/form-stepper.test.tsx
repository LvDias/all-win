import { navigation, renderApp } from "@/test/render";
import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { Input } from "@/shared/ui/input";
import { Label } from "@/shared/ui/label";
import { FormStepper, type FormStep, type SubmitError } from "./form-stepper";

type Values = { kind: "prop" | "real"; name: string; firm: string; note: string };


function Harness({ onSubmit, onCancel = () => {} }: { onSubmit: (values: Values) => Promise<SubmitError[] | void>; onCancel?: () => void }) {
  const form = useForm<Values>({ defaultValues: { kind: "prop", name: "", firm: "", note: "" } });
  const list: FormStep<Values>[] = [
    {
      id: "kind",
      label: "Tipo",
      fields: ["kind"],
      schema: z.object({ kind: z.enum(["prop", "real"]) }),
      content: (
        <div>
          <Label htmlFor="kind">Tipo da conta</Label>
          <Input id="kind" {...form.register("kind")} />
        </div>
      ),
      review: (v) => [{ label: "Tipo", value: v.kind }],
    },
    {
      id: "firm",
      label: "Firma",
      fields: ["firm"],
      when: (v) => v.kind === "prop",
      schema: z.object({ firm: z.string().min(1, "Escolha a firma") }),
      content: (
        <div>
          <Label htmlFor="firm">Firma</Label>
          <Input id="firm" {...form.register("firm")} aria-invalid={form.formState.errors.firm ? true : undefined} />
          {form.formState.errors.firm ? <p>{form.formState.errors.firm.message}</p> : null}
        </div>
      ),
      review: (v) => [{ label: "Firma", value: v.firm }],
    },
    {
      id: "data",
      label: "Dados",
      fields: ["name"],
      schema: z.object({ name: z.string().min(1, "Digite o nome") }),
      content: (
        <div>
          <Label htmlFor="name">Nome</Label>
          <Input id="name" {...form.register("name")} />
          {form.formState.errors.name ? <p>{form.formState.errors.name.message}</p> : null}
        </div>
      ),
      review: (v) => [{ label: "Nome", value: v.name }],
    },
    {
      id: "note",
      label: "Observação",
      optional: true,
      fields: ["note"],
      schema: z.object({ note: z.string() }),
      content: (
        <div>
          <Label htmlFor="note">Observação</Label>
          <Input id="note" {...form.register("note")} />
        </div>
      ),
      review: (v) => [{ label: "Observação", value: v.note }],
    },
  ];
  return <FormStepper form={form} steps={list} onSubmit={onSubmit} submitLabel="Criar conta" onCancel={onCancel} />;
}

async function toReview(person: ReturnType<typeof userEvent.setup>, firm = "Apex") {
  await person.click(screen.getByRole("button", { name: "Continuar" }));
  await person.type(await screen.findByRole("textbox", { name: "Firma" }), firm);
  await person.click(screen.getByRole("button", { name: "Continuar" }));
  await person.type(await screen.findByRole("textbox", { name: "Nome" }), "Conta 1");
  await person.click(screen.getByRole("button", { name: "Continuar" }));
  await person.click(await screen.findByRole("button", { name: "Pular etapa" }));
  await screen.findByRole("heading", { level: 2, name: "Revisar" });
}

beforeEach(() => {
  navigation.search = "";
});

describe("FormStepper", () => {
  it("validates only the current step on Continue and keeps the user there", async () => {
    renderApp(<Harness onSubmit={async () => {}} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    await person.click(await screen.findByRole("button", { name: "Continuar" }));
    expect(await screen.findByText("Escolha a firma")).toBeInTheDocument();
    expect(screen.getByRole("heading", { level: 2, name: "Firma" })).toBeInTheDocument();
    expect(screen.queryByText("Digite o nome")).not.toBeInTheDocument();
  });

  it("Back keeps what was typed", async () => {
    renderApp(<Harness onSubmit={async () => {}} />);
    const person = userEvent.setup();
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    await person.type(await screen.findByRole("textbox", { name: "Firma" }), "Apex");
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    await person.click(await screen.findByRole("button", { name: "Voltar" }));
    expect(await screen.findByRole("textbox", { name: "Firma" })).toHaveValue("Apex");
  });

  it("drops a step that stops applying and lists only the remaining groups in the review", async () => {
    renderApp(<Harness onSubmit={async () => {}} />);
    const person = userEvent.setup();
    const kind = screen.getByRole("textbox", { name: "Tipo da conta" });
    await person.clear(kind);
    await person.type(kind, "real");
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByRole("heading", { level: 2, name: "Dados" })).toBeInTheDocument();
    expect(screen.getAllByText(/Etapa 2 de 4/).length).toBeGreaterThan(0);
  });

  it("shows 'Não definido' for a skipped optional step and Edit returns to the review after Continue", async () => {
    renderApp(<Harness onSubmit={async () => {}} />);
    const person = userEvent.setup();
    await toReview(person);
    const group = screen.getByRole("region", { name: "Observação" });
    expect(within(group).getByText("Não definido")).toBeInTheDocument();
    await person.click(screen.getByRole("button", { name: "Editar Dados" }));
    const name = await screen.findByRole("textbox", { name: "Nome" });
    await person.clear(name);
    await person.type(name, "Outra");
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    expect(await screen.findByRole("heading", { level: 2, name: "Revisar" })).toBeInTheDocument();
    expect(screen.getByText("Outra")).toBeInTheDocument();
  });

  it("sends once on a double click of the final button", async () => {
    const send = vi.fn(() => new Promise<void>((resolve) => window.setTimeout(resolve, 100)));
    renderApp(<Harness onSubmit={send} />);
    const person = userEvent.setup();
    await toReview(person);
    const button = screen.getByRole("button", { name: "Criar conta" });
    await person.dblClick(button);
    await waitFor(() => expect(send).toHaveBeenCalledTimes(1));
  });

  it("sends the form values from the review", async () => {
    const send = vi.fn(async () => {});
    renderApp(<Harness onSubmit={send} />);
    const person = userEvent.setup();
    await toReview(person);
    await person.click(screen.getByRole("button", { name: "Criar conta" }));
    await waitFor(() => expect(send).toHaveBeenCalledWith({ kind: "prop", firm: "Apex", name: "Conta 1", note: "" }));
  });

  it("maps a server pointer to the step that owns the field and links back from the review", async () => {
    renderApp(<Harness onSubmit={async () => [{ pointer: "/name", message: "Nome já usado" }, { message: "Erro geral" }]} />);
    const person = userEvent.setup();
    await toReview(person);
    await person.click(screen.getByRole("button", { name: "Criar conta" }));
    expect(await screen.findByRole("heading", { level: 2, name: "Dados" })).toBeInTheDocument();
    expect(await screen.findByText("Nome já usado")).toBeInTheDocument();
    await person.click(screen.getByRole("button", { name: "Continuar" }));
    // The error stays visible until the user changes the value; the generic one lands in the review.
    expect(await screen.findByRole("heading", { level: 2, name: /Dados|Observação|Revisar/ })).toBeInTheDocument();
  });

  it("asks before discarding a dirty form on the first step", async () => {
    const cancel = vi.fn();
    renderApp(<Harness onSubmit={async () => {}} onCancel={cancel} />);
    const person = userEvent.setup();
    const kind = screen.getByRole("textbox", { name: "Tipo da conta" });
    await person.type(kind, "x");
    await person.click(screen.getByRole("button", { name: "Cancelar" }));
    const dialog = await screen.findByRole("alertdialog");
    expect(cancel).not.toHaveBeenCalled();
    await person.click(within(dialog).getByRole("button", { name: "Descartar" }));
    expect(cancel).toHaveBeenCalledOnce();
  });

  it("cancels at once when nothing changed", async () => {
    const cancel = vi.fn();
    renderApp(<Harness onSubmit={async () => {}} onCancel={cancel} />);
    await userEvent.setup().click(screen.getByRole("button", { name: "Cancelar" }));
    expect(cancel).toHaveBeenCalledOnce();
  });
});
