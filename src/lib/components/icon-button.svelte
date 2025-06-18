<script lang="ts">
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import { Tooltip as TooltipPrimitive } from "bits-ui";
	import { buttonVariants, type ButtonVariant } from "./ui/button";
	import { cn } from "$lib/utils";

    let {
        variant,
        title,
        class: className,
        ...props
    }: TooltipPrimitive.TriggerProps & {
        variant: ButtonVariant,
        class?: string
        title: string
    } = $props();
</script>

<Tooltip.Root>
    <Tooltip.Trigger class={cn(buttonVariants({ variant, size: "icon" }), className)} {...props} aria-label={title} >
    </Tooltip.Trigger>
    <Tooltip.Content interactOutsideBehavior="ignore" side="bottom">
        <span class="select-none">
            {title}
        </span>
    </Tooltip.Content>
</Tooltip.Root>